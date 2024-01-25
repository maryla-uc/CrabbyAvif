#![deny(unsafe_op_in_unsafe_fn)]

pub mod decoder;
pub mod image;
pub mod reformat;
pub mod utils;

#[cfg(feature = "capi")]
pub mod capi;

/// cbindgen:ignore
mod codecs;

mod internal_utils;
mod parser;

#[cfg(feature = "use_ahash")]
type HashMap<K, V> = ahash::AHashMap<K, V>;
#[cfg(feature = "use_ahash")]
type HashSet<K> = ahash::AHashSet<K>;
#[cfg(not(feature = "use_ahash"))]
type HashMap<K, V> = std::collections::HashMap<K, V>;
#[cfg(not(feature = "use_ahash"))]
type HashSet<K> = std::collections::HashSet<K>;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum PixelFormat {
    Yuv444,
    Yuv422,
    #[default]
    Yuv420,
    Monochrome,
}

impl PixelFormat {
    pub fn plane_count(&self) -> usize {
        match self {
            PixelFormat::Monochrome => 1,
            PixelFormat::Yuv420 | PixelFormat::Yuv422 | PixelFormat::Yuv444 => 3,
        }
    }

    pub fn chroma_shift_x(&self) -> u32 {
        match self {
            Self::Yuv422 | Self::Yuv420 => 1,
            _ => 0,
        }
    }

    pub fn chroma_shift_y(&self) -> u32 {
        match self {
            Self::Yuv420 => 1,
            _ => 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum ChromaSamplePosition {
    #[default]
    Unknown = 0,
    Vertical = 1,
    Colocated = 2,
}

impl From<u32> for ChromaSamplePosition {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Vertical,
            2 => Self::Colocated,
            _ => Self::default(),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum ColorPrimaries {
    Unknown = 0,
    Srgb = 1,
    #[default]
    Unspecified = 2,
    Bt470m = 4,
    Bt470bg = 5,
    Bt601 = 6,
    Smpte240 = 7,
    GenericFilm = 8,
    Bt2020 = 9,
    Xyz = 10,
    Smpte431 = 11,
    Smpte432 = 12,
    Ebu3213 = 22,
}

impl From<u16> for ColorPrimaries {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Srgb,
            2 => Self::Unspecified,
            4 => Self::Bt470m,
            5 => Self::Bt470bg,
            6 => Self::Bt601,
            7 => Self::Smpte240,
            8 => Self::GenericFilm,
            9 => Self::Bt2020,
            10 => Self::Xyz,
            11 => Self::Smpte431,
            12 => Self::Smpte432,
            22 => Self::Ebu3213,
            _ => Self::default(),
        }
    }
}

#[allow(non_camel_case_types, non_upper_case_globals)]
impl ColorPrimaries {
    pub const Bt709: Self = Self::Srgb;
    pub const Iec61966_2_4: Self = Self::Srgb;
    pub const Bt2100: Self = Self::Bt2020;
    pub const Dci_p3: Self = Self::Smpte432;
}

#[repr(u16)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum TransferCharacteristics {
    Unknown = 0,
    Bt709 = 1,
    #[default]
    Unspecified = 2,
    Bt470m = 4,  // 2.2 gamma
    Bt470bg = 5, // 2.8 gamma
    Bt601 = 6,
    Smpte240 = 7,
    Linear = 8,
    Log100 = 9,
    Log100Sqrt10 = 10,
    Iec61966 = 11,
    Bt1361 = 12,
    Srgb = 13,
    Bt2020_10bit = 14,
    Bt2020_12bit = 15,
    Pq = 16, // Perceptual Quantizer (HDR); BT.2100 PQ
    Smpte428 = 17,
    Hlg = 18, // Hybrid Log-Gamma (HDR); ARIB STD-B67; BT.2100 HLG
}

impl From<u16> for TransferCharacteristics {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Bt709,
            2 => Self::Unspecified,
            4 => Self::Bt470m,
            5 => Self::Bt470bg,
            6 => Self::Bt601,
            7 => Self::Smpte240,
            8 => Self::Linear,
            9 => Self::Log100,
            10 => Self::Log100Sqrt10,
            11 => Self::Iec61966,
            12 => Self::Bt1361,
            13 => Self::Srgb,
            14 => Self::Bt2020_10bit,
            15 => Self::Bt2020_12bit,
            16 => Self::Pq,
            17 => Self::Smpte428,
            18 => Self::Hlg,
            _ => Self::default(),
        }
    }
}

#[allow(non_upper_case_globals)]
impl TransferCharacteristics {
    pub const Smpte2084: Self = Self::Pq;
}

#[repr(u16)]
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
pub enum MatrixCoefficients {
    Identity = 0,
    Bt709 = 1,
    #[default]
    Unspecified = 2,
    Fcc = 4,
    Bt470bg = 5,
    Bt601 = 6,
    Smpte240 = 7,
    Ycgco = 8,
    Bt2020Ncl = 9,
    Bt2020Cl = 10,
    Smpte2085 = 11,
    ChromaDerivedNcl = 12,
    ChromaDerivedCl = 13,
    Ictcp = 14,
    YcgcoRe = 15,
    YcgcoRo = 16,
}

impl From<u16> for MatrixCoefficients {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Identity,
            1 => Self::Bt709,
            2 => Self::Unspecified,
            4 => Self::Fcc,
            5 => Self::Bt470bg,
            6 => Self::Bt601,
            7 => Self::Smpte240,
            8 => Self::Ycgco,
            9 => Self::Bt2020Ncl,
            10 => Self::Bt2020Cl,
            11 => Self::Smpte2085,
            12 => Self::ChromaDerivedNcl,
            13 => Self::ChromaDerivedCl,
            14 => Self::Ictcp,
            15 => Self::YcgcoRe,
            16 => Self::YcgcoRo,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum AvifError {
    #[default]
    Ok,
    UnknownError,
    InvalidFtyp,
    NoContent,
    NoYuvFormatSelected,
    ReformatFailed,
    UnsupportedDepth,
    EncodeColorFailed,
    EncodeAlphaFailed,
    BmffParseFailed, // TODO: this can contain an error string?
    MissingImageItem,
    DecodeColorFailed,
    DecodeAlphaFailed,
    ColorAlphaSizeMismatch,
    IspeSizeMismatch,
    NoCodecAvailable,
    NoImagesRemaining,
    InvalidExifPayload,
    InvalidImageGrid,
    InvalidCodecSpecificOption,
    TruncatedData,
    IoNotSet,
    IoError,
    WaitingOnIo,
    InvalidArgument,
    NotImplemented,
    OutOfMemory,
    CannotChangeSetting,
    IncompatibleImage,
    EncodeGainMapFailed,
    DecodeGainMapFailed,
    InvalidToneMappedImage,
}

pub type AvifResult<T> = Result<T, AvifError>;
