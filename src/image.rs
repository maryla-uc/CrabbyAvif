use crate::decoder::tile::TileInfo;
use crate::decoder::ProgressiveState;
use crate::internal_utils::*;
use crate::parser::mp4box::*;
use crate::*;

use num_derive::ToPrimitive;
use num_traits::cast::ToPrimitive;

// TODO: needed only for debug to Image and PlaneData. Can be removed it those
// do not have to be debug printable.
use derivative::Derivative;

#[derive(PartialEq, ToPrimitive, Copy, Clone)]
pub enum Plane {
    Y = 0,
    U = 1,
    V = 2,
    A = 3,
}

pub const MAX_PLANE_COUNT: usize = 4;
pub const YUV_PLANES: [Plane; 3] = [Plane::Y, Plane::U, Plane::V];
pub const A_PLANE: [Plane; 1] = [Plane::A];
pub const ALL_PLANES: [Plane; MAX_PLANE_COUNT] = [Plane::Y, Plane::U, Plane::V, Plane::A];

#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub depth: u8,

    pub yuv_format: PixelFormat,
    pub full_range: bool,
    pub chroma_sample_position: ChromaSamplePosition,

    pub alpha_present: bool,
    pub alpha_premultiplied: bool,

    pub planes: [Option<*const u8>; MAX_PLANE_COUNT],
    pub row_bytes: [u32; MAX_PLANE_COUNT],
    pub image_owns_planes: bool,
    pub image_owns_alpha_plane: bool,
    #[derivative(Debug = "ignore")]
    plane_buffers: [Vec<u8>; MAX_PLANE_COUNT],

    pub color_primaries: ColorPrimaries,
    pub transfer_characteristics: TransferCharacteristics,
    pub matrix_coefficients: MatrixCoefficients,

    pub clli: Option<ContentLightLevelInformation>,
    pub pasp: Option<PixelAspectRatio>,
    pub clap: Option<CleanAperture>,
    pub irot_angle: Option<u8>,
    pub imir_axis: Option<u8>,

    #[derivative(Debug = "ignore")]
    pub exif: Vec<u8>,
    #[derivative(Debug = "ignore")]
    pub icc: Vec<u8>,
    #[derivative(Debug = "ignore")]
    pub xmp: Vec<u8>,

    pub image_sequence_track_present: bool,
    pub progressive_state: ProgressiveState,
    // TODO: gainmap image ?
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct PlaneData<'a> {
    #[derivative(Debug = "ignore")]
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub row_bytes: u32,
    pub pixel_size: u32,
}

impl Image {
    pub fn subsampled_width(&self, width: u32, plane: Plane) -> usize {
        match plane {
            Plane::Y | Plane::A => width as usize,
            _ => match self.yuv_format {
                PixelFormat::Yuv444 | PixelFormat::Monochrome => width as usize,
                PixelFormat::Yuv420 | PixelFormat::Yuv422 => (width as usize + 1) / 2,
            },
        }
    }

    pub fn width(&self, plane: Plane) -> usize {
        self.subsampled_width(self.width, plane)
    }

    pub fn height(&self, plane: Plane) -> usize {
        match plane {
            Plane::Y | Plane::A => self.height as usize,
            _ => match self.yuv_format {
                PixelFormat::Yuv444 | PixelFormat::Monochrome | PixelFormat::Yuv422 => {
                    self.height as usize
                }
                PixelFormat::Yuv420 => (self.height as usize + 1) / 2,
            },
        }
    }

    pub fn plane(&self, plane: Plane) -> Option<PlaneData> {
        let plane_index = plane.to_usize().unwrap();
        self.planes[plane_index]?;
        let pixel_size = if self.depth == 8 { 1 } else { 2 };
        let height = self.height(plane);
        let row_bytes = self.row_bytes[plane_index] as usize;
        let plane_size = height * row_bytes;
        let data =
            unsafe { std::slice::from_raw_parts(self.planes[plane_index].unwrap(), plane_size) };
        Some(PlaneData {
            data,
            width: self.width(plane) as u32,
            height: height as u32,
            row_bytes: row_bytes as u32,
            pixel_size,
        })
    }

    pub fn allocate_planes(&mut self, category: usize) -> AvifResult<()> {
        // TODO : assumes 444. do other stuff.
        // TODO: do not realloc if size is already big enough.
        let pixel_size: u32 = if self.depth == 8 { 1 } else { 2 };
        let plane_size = (self.width * self.height * pixel_size) as usize;
        if category == 0 || category == 2 {
            for plane_index in 0usize..3 {
                self.plane_buffers[plane_index].reserve(plane_size);
                self.plane_buffers[plane_index].resize(plane_size, 0);
                self.row_bytes[plane_index] = self.width * pixel_size;
                self.planes[plane_index] = Some(self.plane_buffers[plane_index].as_ptr());
            }
            self.image_owns_planes = true;
        } else {
            assert!(category == 1);
            self.plane_buffers[3].reserve(plane_size);
            self.plane_buffers[3].resize(plane_size, 255);
            self.row_bytes[3] = self.width * pixel_size;
            self.planes[3] = Some(self.plane_buffers[3].as_ptr());
            self.image_owns_alpha_plane = true;
        }
        Ok(())
    }

    pub fn copy_from_slice(
        &mut self,
        source: &[u8],
        stride: u32,
        category: usize,
    ) -> AvifResult<()> {
        // TODO: deal with integer math safety in this function.
        self.allocate_planes(category)?;
        let pixel_size: usize = if self.depth == 8 { 1 } else { 2 };
        // TODO: if width == stride, the whole thing can be one copy.
        if category == 0 || category == 2 {
            let mut src_offset = 0;
            for plane in YUV_PLANES {
                let plane_stride = self.subsampled_width(stride, plane);
                let width = self.width(plane);
                let height = self.height(plane);
                let row_width = width * pixel_size;
                let mut dst_offset = 0;
                let plane_index = plane.to_usize().unwrap();
                for _y in 0usize..height {
                    let src_y_start = src_offset;
                    let src_y_end = src_y_start + row_width;
                    let src_slice = &source[src_y_start..src_y_end];

                    let dst_y_start = dst_offset;
                    let dst_y_end = dst_y_start + row_width;
                    let dst_slice = &mut self.plane_buffers[plane_index][dst_y_start..dst_y_end];

                    dst_slice.copy_from_slice(src_slice);

                    src_offset += plane_stride;
                    dst_offset += self.row_bytes[plane_index] as usize;
                }
            }
        } else {
            assert!(category == 1);
            let mut src_offset = 0;
            let width = self.width(Plane::A);
            let height = self.height(Plane::A);
            let row_width = width * pixel_size;
            let mut dst_offset = 0;
            for _y in 0usize..height {
                let src_y_start = src_offset;
                let src_y_end = src_y_start + row_width;
                let src_slice = &source[src_y_start..src_y_end];

                let dst_y_start = dst_offset;
                let dst_y_end = dst_y_start + row_width;
                let dst_slice = &mut self.plane_buffers[3][dst_y_start..dst_y_end];

                dst_slice.copy_from_slice(src_slice);

                src_offset += stride as usize;
                dst_offset += self.row_bytes[3] as usize;
            }
        }
        Ok(())
    }

    pub fn steal_from(&mut self, src: &Image, category: usize) {
        match category {
            0 | 2 => {
                self.planes[0] = src.planes[0];
                self.planes[1] = src.planes[1];
                self.planes[2] = src.planes[2];
                self.row_bytes[0] = src.row_bytes[0];
                self.row_bytes[1] = src.row_bytes[1];
                self.row_bytes[2] = src.row_bytes[2];
            }
            1 => {
                self.planes[3] = src.planes[3];
                self.row_bytes[3] = src.row_bytes[3];
            }
            _ => {
                panic!("invalid category in steal planes");
            }
        }
    }

    pub fn copy_from_tile(
        &mut self,
        tile: &Image,
        tile_info: &TileInfo,
        tile_index: u32,
        category: usize,
    ) -> AvifResult<()> {
        let err = AvifError::BmffParseFailed;
        let row_index = u64::from(tile_index / tile_info.grid.columns);
        let column_index = u64::from(tile_index % tile_info.grid.columns);
        //println!("copying tile {tile_index} {row_index} {column_index}");

        let planes: &[Plane] = if category == 1 { &A_PLANE } else { &YUV_PLANES };
        for plane in planes {
            let plane = *plane;
            let src_plane = tile.plane(plane);
            if src_plane.is_none() {
                continue;
            }
            let plane_index = plane.to_usize().unwrap();
            let src_plane = src_plane.unwrap();
            // If this is the last tile column, clamp to left over width.
            let src_width_to_copy = if column_index == (tile_info.grid.columns - 1).into() {
                let width_so_far = u64::from(src_plane.width)
                    .checked_mul(column_index)
                    .ok_or(err)?;
                u64_from_usize(self.width(plane))?
                    .checked_sub(width_so_far)
                    .ok_or(err)?
            } else {
                u64::from(src_plane.width)
            };
            //println!("src_width_to_copy: {src_width_to_copy}");
            let src_byte_count = src_width_to_copy * u64::from(src_plane.pixel_size);
            let dst_row_bytes = u64::from(self.row_bytes[plane_index]);
            let dst_base_offset = (row_index * (u64::from(src_plane.height) * dst_row_bytes))
                + (column_index * u64::from(src_plane.width * src_plane.pixel_size));
            //println!("dst base_offset: {dst_base_offset}");

            // If this is the last tile row, clamp to left over height.
            let src_height_to_copy = if row_index == (tile_info.grid.rows - 1).into() {
                let height_so_far = u64::from(src_plane.height)
                    .checked_mul(row_index)
                    .ok_or(err)?;
                u64_from_usize(self.height(plane))?
                    .checked_sub(height_so_far)
                    .ok_or(err)?
            } else {
                u64::from(src_plane.height)
            };

            //println!("src_height_to_copy: {src_height_to_copy}");
            for y in 0..src_height_to_copy {
                let src_stride_offset = y.checked_mul(u64::from(src_plane.row_bytes)).ok_or(err)?;
                let src_end_offset = src_stride_offset.checked_add(src_byte_count).ok_or(err)?;
                let dst_row_offset = y.checked_mul(dst_row_bytes).ok_or(err)?;
                let dst_stride_offset = dst_base_offset.checked_add(dst_row_offset).ok_or(err)?;
                let dst_end_offset = dst_stride_offset.checked_add(src_byte_count).ok_or(err)?;

                let src_slice = &src_plane.data
                    [usize_from_u64(src_stride_offset)?..usize_from_u64(src_end_offset)?];
                let dst_slice = &mut self.plane_buffers[plane_index]
                    [usize_from_u64(dst_stride_offset)?..usize_from_u64(dst_end_offset)?];
                dst_slice.copy_from_slice(src_slice);
            }
        }
        Ok(())
    }
}
