use super::coeffs::*;
use super::rgb;
use super::rgb::*;

use crate::image;
use crate::image::Plane;
use crate::internal_utils::*;
use crate::*;

use std::cmp::min;

#[allow(unused)]
struct RgbColorSpaceInfo {
    channel_bytes: u32,
    pixel_bytes: u32,
    offset_bytes_r: usize,
    offset_bytes_g: usize,
    offset_bytes_b: usize,
    offset_bytes_a: usize,
    max_channel: i32,
    max_channel_f: f32,
}

impl RgbColorSpaceInfo {
    fn create_from(rgb: &Image) -> AvifResult<Self> {
        if !rgb.depth_valid()
            || (rgb.is_float && rgb.depth != 16)
            || (rgb.format == Format::Rgb565 && rgb.depth != 8)
        {
            return Err(AvifError::ReformatFailed);
        }
        let offsets = rgb.format.offsets();
        let max_channel = i32_from_u32((1 << rgb.depth) - 1)?;
        Ok(Self {
            channel_bytes: rgb.channel_size(),
            pixel_bytes: rgb.pixel_size(),
            offset_bytes_r: (rgb.channel_size() as usize * offsets[0]),
            offset_bytes_g: (rgb.channel_size() as usize * offsets[1]),
            offset_bytes_b: (rgb.channel_size() as usize * offsets[2]),
            offset_bytes_a: (rgb.channel_size() as usize * offsets[3]),
            max_channel,
            max_channel_f: max_channel as f32,
        })
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    YuvCoefficients(f32, f32, f32),
    Identity,
    Ycgco,
}

#[allow(unused)]
struct YuvColorSpaceInfo {
    channel_bytes: u32,
    depth: u32,
    full_range: bool,
    max_channel: u16,
    bias_y: f32,
    bias_uv: f32,
    range_y: f32,
    range_uv: f32,
    format: PixelFormat,
    mode: Mode,
}

impl YuvColorSpaceInfo {
    fn create_from(image: &image::Image) -> AvifResult<Self> {
        if !image.depth_valid() {
            return Err(AvifError::ReformatFailed);
        }
        // Unsupported matrix coefficients.
        match image.matrix_coefficients {
            MatrixCoefficients::Ycgco
            | MatrixCoefficients::Bt2020Cl
            | MatrixCoefficients::Smpte2085
            | MatrixCoefficients::ChromaDerivedCl
            | MatrixCoefficients::Ictcp => return Err(AvifError::ReformatFailed),
            _ => {}
        }
        if image.matrix_coefficients == MatrixCoefficients::Identity
            && image.yuv_format != PixelFormat::Yuv444
            && image.yuv_format != PixelFormat::Monochrome
        {
            return Err(AvifError::ReformatFailed);
        }
        let mode = match image.matrix_coefficients {
            MatrixCoefficients::Identity => Mode::Identity,
            MatrixCoefficients::Ycgco => Mode::Ycgco,
            _ => {
                let coeffs =
                    calculate_yuv_coefficients(image.color_primaries, image.matrix_coefficients);
                Mode::YuvCoefficients(coeffs[0], coeffs[1], coeffs[2])
            }
        };
        let max_channel = (1 << image.depth) - 1;
        Ok(Self {
            channel_bytes: if image.depth == 8 { 1 } else { 2 },
            depth: image.depth as u32,
            full_range: image.full_range,
            max_channel,
            bias_y: if image.full_range { 0.0 } else { (16 << (image.depth - 8)) as f32 },
            bias_uv: (1 << (image.depth - 1)) as f32,
            range_y: if image.full_range { max_channel } else { 219 << (image.depth - 8) } as f32,
            range_uv: if image.full_range { max_channel } else { 224 << (image.depth - 8) } as f32,
            format: image.yuv_format,
            mode,
        })
    }
}

struct State {
    #[allow(unused)]
    rgb: RgbColorSpaceInfo,
    #[allow(unused)]
    yuv: YuvColorSpaceInfo,
}

fn identity_yuv8_to_rgb8_full_range(image: &image::Image, rgb: &mut rgb::Image) -> AvifResult<()> {
    let r_offset = rgb.format.r_offset();
    let g_offset = rgb.format.g_offset();
    let b_offset = rgb.format.b_offset();
    let rgb565 = rgb.format == Format::Rgb565;
    let channel_count = rgb.channel_count() as usize;
    for i in 0..image.height {
        let y = image.row(Plane::Y, i)?;
        let u = image.row(Plane::U, i)?;
        let v = image.row(Plane::V, i)?;
        let rgb_pixels = rgb.row_mut(i)?;
        if rgb565 {
            unimplemented!("rgb 565 is not implemented");
        } else {
            for j in 0..image.width as usize {
                rgb_pixels[(j * channel_count) + r_offset] = v[j];
                rgb_pixels[(j * channel_count) + g_offset] = y[j];
                rgb_pixels[(j * channel_count) + b_offset] = u[j];
            }
        }
    }
    Ok(())
}

pub fn yuv_to_rgb(image: &image::Image, rgb: &mut rgb::Image) -> AvifResult<()> {
    // TODO: This function is equivalent to libavif's "fast" path. Implement the slow path too.
    let state = State {
        rgb: RgbColorSpaceInfo::create_from(rgb)?,
        yuv: YuvColorSpaceInfo::create_from(image)?,
    };
    if state.yuv.mode == Mode::Identity {
        if image.depth == 8
            && rgb.depth == 8
            && image.yuv_format == PixelFormat::Yuv444
            && image.full_range
        {
            return identity_yuv8_to_rgb8_full_range(image, rgb);
        }
        // TODO: Add more fast paths for identity.
        return Err(AvifError::NotImplemented);
    }
    Err(AvifError::NotImplemented)
}

fn unorm_lookup_tables(depth: u8, state: &State) -> (Vec<f32>, Vec<f32>) {
    let count = (1i32 << depth) as usize;
    let mut table_y: Vec<f32> = Vec::new();
    for cp in 0..count {
        table_y.push(((cp as f32) - state.yuv.bias_y) / state.yuv.range_y);
    }
    let mut table_uv: Vec<f32> = Vec::new();
    if state.yuv.mode == Mode::Identity {
        table_uv.extend_from_slice(&table_y[..]);
    } else {
        for cp in 0..count {
            table_uv.push(((cp as f32) - state.yuv.bias_uv) / state.yuv.range_uv);
        }
    }
    (table_y, table_uv)
}

fn compute_rgb(Y: f32, Cb: f32, Cr: f32, has_color: bool, mode: Mode) -> (f32, f32, f32) {
    let R: f32;
    let G: f32;
    let B: f32;
    if has_color {
        match mode {
            Mode::Identity => {
                G = Y;
                B = Cb;
                R = Cr;
            }
            Mode::Ycgco => {
                let t = Y - Cb;
                G = Y + Cb;
                B = t - Cr;
                R = t + Cr;
            }
            Mode::YuvCoefficients(kr, kg, kb) => {
                R = Y + (2.0 * (1.0 - kr)) * Cr;
                B = Y + (2.0 * (1.0 - kb)) * Cb;
                G = Y - ((2.0 * ((kr * (1.0 - kr) * Cr) + (kb * (1.0 - kb) * Cb))) / kg);
            }
        }
    } else {
        R = Y;
        G = Y;
        B = Y;
    }
    (
        clamp_f32(R, 0.0, 1.0),
        clamp_f32(G, 0.0, 1.0),
        clamp_f32(B, 0.0, 1.0),
    )
}

pub fn yuv_to_rgb_any(
    image: &image::Image,
    rgb: &mut rgb::Image,
    alpha_multiply_mode: AlphaMultiplyMode,
) -> AvifResult<()> {
    let state = State {
        rgb: RgbColorSpaceInfo::create_from(rgb)?,
        yuv: YuvColorSpaceInfo::create_from(image)?,
    };
    let (table_y, table_uv) = unorm_lookup_tables(image.depth, &state);
    let yuv_channel_bytes = state.yuv.channel_bytes;
    let rgb_pixel_bytes = state.rgb.pixel_bytes;
    let r_offset = rgb.format.r_offset();
    let g_offset = rgb.format.g_offset();
    let b_offset = rgb.format.b_offset();
    let rgb_channel_count = rgb.channel_count() as usize;
    let chroma_upsampling = rgb.chroma_upsampling;
    let has_color = image.has_plane(Plane::U)
        && image.has_plane(Plane::V)
        && image.yuv_format != PixelFormat::Monochrome;
    let yuv_max_channel = state.yuv.max_channel;
    let rgb_max_channel_f = state.rgb.max_channel_f;
    if image.depth > 8 && rgb.depth > 8 {
        for j in 0..image.height as usize {
            let uv_j = j >> image.yuv_format.chroma_shift_y();
            let y_row = image.row16(Plane::Y, j as u32)?;
            let u_row = image.row16(Plane::U, uv_j as u32);
            let v_row = image.row16(Plane::V, uv_j as u32);
            let rgb_row = rgb.row16_mut(j as u32)?;
            for i in 0..image.width as usize {
                let Y = table_y[min(yuv_max_channel, y_row[i]) as usize];
                let mut Cb = 0.5;
                let mut Cr = 0.5;
                if has_color {
                    let u_row = u_row.as_ref().unwrap();
                    let v_row = v_row.as_ref().unwrap();
                    let uv_i = i >> image.yuv_format.chroma_shift_x();
                    if image.yuv_format == PixelFormat::Yuv444
                        || matches!(
                            chroma_upsampling,
                            ChromaUpsampling::Fastest | ChromaUpsampling::Nearest
                        )
                    {
                        Cb = table_uv[min(yuv_max_channel, u_row[uv_i]) as usize];
                        Cr = table_uv[min(yuv_max_channel, v_row[uv_i]) as usize];
                    } else {
                        // Bilinear filtering with weights.
                        let image_width_minus_1 = (image.width - 1) as usize;
                        let uv_adj_col: i32 =
                            if i == 0 || (i == image_width_minus_1 && (i % 2) != 0) {
                                0
                            } else {
                                if (i % 2) != 0 {
                                    1
                                } else {
                                    -1
                                }
                            };
                        let u_adj_row;
                        let v_adj_row;
                        let image_height_minus_1 = (image.height - 1) as usize;
                        if j == 0
                            || (j != image_height_minus_1 && (j % 2) != 0)
                            || image.yuv_format == PixelFormat::Yuv422
                        {
                            u_adj_row = *u_row;
                            v_adj_row = *v_row;
                        } else {
                            if (j % 2) != 0 {
                                u_adj_row = image.row16(Plane::U, (uv_j + 1) as u32)?;
                                v_adj_row = image.row16(Plane::V, (uv_j + 1) as u32)?;
                            } else {
                                u_adj_row = image.row16(Plane::U, (uv_j - 1) as u32)?;
                                v_adj_row = image.row16(Plane::V, (uv_j - 1) as u32)?;
                            }
                        }
                        let mut unorm_u: [[u16; 2]; 2] = [[0; 2]; 2];
                        let mut unorm_v: [[u16; 2]; 2] = [[0; 2]; 2];
                        unorm_u[0][0] = u_row[uv_i];
                        unorm_v[0][0] = v_row[uv_i];
                        unorm_u[1][0] = u_row[((uv_i as i32) + uv_adj_col) as usize];
                        unorm_v[1][0] = v_row[((uv_i as i32) + uv_adj_col) as usize];
                        unorm_u[0][1] = u_adj_row[uv_i];
                        unorm_v[0][1] = v_adj_row[uv_i];
                        unorm_u[1][1] = u_adj_row[((uv_i as i32) + uv_adj_col) as usize];
                        unorm_v[1][1] = v_adj_row[((uv_i as i32) + uv_adj_col) as usize];
                        for yy in 0..2 {
                            for xx in 0..2 {
                                unorm_u[yy][xx] = min(yuv_max_channel, unorm_u[yy][xx]);
                                unorm_v[yy][xx] = min(yuv_max_channel, unorm_v[yy][xx]);
                            }
                        }
                        Cb = (table_uv[unorm_u[0][0] as usize] * (9.0 / 16.0))
                            + (table_uv[unorm_u[1][0] as usize] * (3.0 / 16.0))
                            + (table_uv[unorm_u[0][1] as usize] * (3.0 / 16.0))
                            + (table_uv[unorm_u[1][1] as usize] * (1.0 / 16.0));
                        Cr = (table_uv[unorm_v[0][0] as usize] * (9.0 / 16.0))
                            + (table_uv[unorm_v[1][0] as usize] * (3.0 / 16.0))
                            + (table_uv[unorm_v[0][1] as usize] * (3.0 / 16.0))
                            + (table_uv[unorm_v[1][1] as usize] * (1.0 / 16.0));
                    }
                }
                let (Rc, Gc, Bc) = compute_rgb(Y, Cb, Cr, has_color, state.yuv.mode);

                // TODO: handle alpha multiply mode.

                rgb_row[(i * rgb_channel_count) + r_offset] =
                    (0.5 + (Rc * rgb_max_channel_f)) as u16;
                rgb_row[(i * rgb_channel_count) + g_offset] =
                    (0.5 + (Gc * rgb_max_channel_f)) as u16;
                rgb_row[(i * rgb_channel_count) + b_offset] =
                    (0.5 + (Bc * rgb_max_channel_f)) as u16;
            }
        }
        return Ok(());
    }
    unimplemented!("in any alpha mode: {:#?}!", alpha_multiply_mode);
}
