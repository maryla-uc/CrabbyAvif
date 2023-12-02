use crate::decoder::Image;
use crate::parser::mp4box::ContentLightLevelInformation;
use crate::utils::*;

#[derive(Default, Debug)]
pub struct GainMapMetadata {
    pub min: [Fraction; 3],
    pub max: [Fraction; 3],
    pub gamma: [UFraction; 3],
    pub base_offset: [Fraction; 3],
    pub alternate_offset: [Fraction; 3],
    pub base_hdr_headroom: UFraction,
    pub alternate_hdr_headroom: UFraction,
    pub backward_direction: bool,
    pub use_base_color_space: bool,
}

#[derive(Default, Debug)]
pub struct GainMap {
    pub image: Image,
    pub metadata: GainMapMetadata,

    pub alt_icc: Vec<u8>,
    pub alt_color_primaries: u16,
    pub alt_transfer_characteristics: u16,
    pub alt_matrix_coefficients: u16,
    pub alt_full_range: bool,

    pub alt_clli: ContentLightLevelInformation,

    pub alt_plane_count: u8,
    pub alt_plane_depth: u8,
}
