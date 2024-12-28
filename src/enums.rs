use bitmask_enum::bitmask;

#[derive(Debug, Default)]
pub enum VipsAccess {
    #[default]
    Random,
    Sequential,
    SequentialUnbuffered,
    Last
}

impl Into<crate::bindings::VipsAccess> for VipsAccess {
    fn into(self) -> crate::bindings::VipsAccess {
        match self {
            VipsAccess::Random => crate::bindings::VipsAccess_VIPS_ACCESS_RANDOM,
            VipsAccess::Sequential => crate::bindings::VipsAccess_VIPS_ACCESS_SEQUENTIAL,
            VipsAccess::SequentialUnbuffered => crate::bindings::VipsAccess_VIPS_ACCESS_SEQUENTIAL_UNBUFFERED,
            VipsAccess::Last => crate::bindings::VipsAccess_VIPS_ACCESS_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsPCS {
    #[default]
    Lab,
    XYZ,
    Last
}

impl Into<crate::bindings::VipsPCS> for VipsPCS {
    fn into(self) -> crate::bindings::VipsPCS {
        match self {
            VipsPCS::Lab => crate::bindings::VipsPCS_VIPS_PCS_LAB,
            VipsPCS::XYZ => crate::bindings::VipsPCS_VIPS_PCS_XYZ,
            VipsPCS::Last => crate::bindings::VipsPCS_VIPS_PCS_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsIntent {
    #[default]
    Relative,
    Perceptual,
    Saturation,
    Absolute,
    Last
}

impl Into<crate::bindings::VipsIntent> for VipsIntent {
    fn into(self) -> crate::bindings::VipsIntent {
        match self {
            VipsIntent::Relative => crate::bindings::VipsIntent_VIPS_INTENT_RELATIVE,
            VipsIntent::Perceptual => crate::bindings::VipsIntent_VIPS_INTENT_PERCEPTUAL,
            VipsIntent::Saturation => crate::bindings::VipsIntent_VIPS_INTENT_SATURATION,
            VipsIntent::Absolute => crate::bindings::VipsIntent_VIPS_INTENT_ABSOLUTE,
            VipsIntent::Last => crate::bindings::VipsIntent_VIPS_INTENT_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsAngle {
    #[default]
    None,
    Left,
    UpsideDown,
    Right,
    Last
}

impl Into<crate::bindings::VipsAngle> for VipsAngle {
    fn into(self) -> crate::bindings::VipsAngle {
        match self {
            VipsAngle::None => crate::bindings::VipsAngle_VIPS_ANGLE_D0,
            VipsAngle::Left => crate::bindings::VipsAngle_VIPS_ANGLE_D270,
            VipsAngle::UpsideDown => crate::bindings::VipsAngle_VIPS_ANGLE_D180,
            VipsAngle::Right => crate::bindings::VipsAngle_VIPS_ANGLE_D90,
            VipsAngle::Last => crate::bindings::VipsAngle_VIPS_ANGLE_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsBlendMode {
    #[default]
    Over,
    Clear,
    Source,
    In,
    Out,
    Atop,
    Dest,
    DestOver,
    DestIn,
    DestOut,
    DestAtop,
    Xor,
    Add,
    Saturate,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColourDodge,
    ColourBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Last
}

impl Into<crate::bindings::VipsBlendMode> for VipsBlendMode {
    fn into(self) -> crate::bindings::VipsBlendMode {
        match self {
            VipsBlendMode::Over => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_OVER,
            VipsBlendMode::Clear => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_CLEAR,
            VipsBlendMode::Source => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_SOURCE,
            VipsBlendMode::In => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_IN,
            VipsBlendMode::Out => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_OUT,
            VipsBlendMode::Atop => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_ATOP,
            VipsBlendMode::Dest => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DEST,
            VipsBlendMode::DestOver => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DEST_OVER,
            VipsBlendMode::DestIn => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DEST_IN,
            VipsBlendMode::DestOut => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DEST_OUT,
            VipsBlendMode::DestAtop => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DEST_ATOP,
            VipsBlendMode::Xor => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_XOR,
            VipsBlendMode::Add => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_ADD,
            VipsBlendMode::Saturate => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_SATURATE,
            VipsBlendMode::Multiply => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_MULTIPLY,
            VipsBlendMode::Screen => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_SCREEN,
            VipsBlendMode::Overlay => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_OVERLAY,
            VipsBlendMode::Darken => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DARKEN,
            VipsBlendMode::Lighten => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_LIGHTEN,
            VipsBlendMode::ColourDodge => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_COLOUR_DODGE,
            VipsBlendMode::ColourBurn => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_COLOUR_BURN,
            VipsBlendMode::HardLight => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_HARD_LIGHT,
            VipsBlendMode::SoftLight => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_SOFT_LIGHT,
            VipsBlendMode::Difference => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_DIFFERENCE,
            VipsBlendMode::Exclusion => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_EXCLUSION,
            VipsBlendMode::Last => crate::bindings::VipsBlendMode_VIPS_BLEND_MODE_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsInterpretation {
    #[default]
    #[allow(warnings)]
    sRGB,
    Error,
    MultiBand,
    BlackWhite,
    Histogram,
    XYZ,
    Lab,
    CMYK,
    LabQ,
    RGB,
    CMC,
    LCh,
    LabS,
    Yxy,
    Fourier,
    RGB16,
    GREY16,
    Matrix,
    #[allow(warnings)]
    scRGB,
    HSV,
    Last
}

impl Into<crate::bindings::VipsInterpretation> for VipsInterpretation {
    fn into(self) -> crate::bindings::VipsInterpretation {
        match self {
            VipsInterpretation::sRGB => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_sRGB,
            VipsInterpretation::Error => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_ERROR,
            VipsInterpretation::MultiBand => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_MULTIBAND,
            VipsInterpretation::BlackWhite => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_B_W,
            VipsInterpretation::Histogram => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_HISTOGRAM,
            VipsInterpretation::XYZ => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_XYZ,
            VipsInterpretation::Lab => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_LAB,
            VipsInterpretation::CMYK => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_CMYK,
            VipsInterpretation::LabQ => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_LABQ,
            VipsInterpretation::RGB => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_RGB,
            VipsInterpretation::CMC => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_CMC,
            VipsInterpretation::LCh => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_LCH,
            VipsInterpretation::LabS => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_LABS,
            VipsInterpretation::Yxy => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_YXY,
            VipsInterpretation::Fourier => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_FOURIER,
            VipsInterpretation::RGB16 => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_RGB16,
            VipsInterpretation::GREY16 => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_GREY16,
            VipsInterpretation::Matrix => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_MATRIX,
            VipsInterpretation::scRGB => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_scRGB,
            VipsInterpretation::HSV => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_HSV,
            VipsInterpretation::Last => crate::bindings::VipsInterpretation_VIPS_INTERPRETATION_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsSize {
    #[default]
    Both,
    Up,
    Down,
    Force,
    Last
}

impl Into<crate::bindings::VipsSize> for VipsSize {
    fn into(self) -> crate::bindings::VipsSize {
        match self {
            VipsSize::Both => crate::bindings::VipsSize_VIPS_SIZE_BOTH,
            VipsSize::Up => crate::bindings::VipsSize_VIPS_SIZE_UP,
            VipsSize::Down => crate::bindings::VipsSize_VIPS_SIZE_DOWN,
            VipsSize::Force => crate::bindings::VipsSize_VIPS_SIZE_FORCE,
            VipsSize::Last => crate::bindings::VipsSize_VIPS_SIZE_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsInteresting {
    #[default]
    None,
    Centre,
    Entropy,
    Attention,
    Low,
    High,
    All,
    Last
}

impl Into<crate::bindings::VipsInteresting> for VipsInteresting {
    fn into(self) -> crate::bindings::VipsInteresting {
        match self {
            VipsInteresting::None => crate::bindings::VipsInteresting_VIPS_INTERESTING_NONE,
            VipsInteresting::Centre => crate::bindings::VipsInteresting_VIPS_INTERESTING_CENTRE,
            VipsInteresting::Entropy => crate::bindings::VipsInteresting_VIPS_INTERESTING_ENTROPY,
            VipsInteresting::Attention => crate::bindings::VipsInteresting_VIPS_INTERESTING_ATTENTION,
            VipsInteresting::Low => crate::bindings::VipsInteresting_VIPS_INTERESTING_LOW,
            VipsInteresting::High => crate::bindings::VipsInteresting_VIPS_INTERESTING_HIGH,
            VipsInteresting::All => crate::bindings::VipsInteresting_VIPS_INTERESTING_ALL,
            VipsInteresting::Last => crate::bindings::VipsInteresting_VIPS_INTERESTING_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsFailOn {
    #[default]
    None,
    Truncated,
    Error,
    Warning,
    Last
}

impl Into<crate::bindings::VipsFailOn> for VipsFailOn {
    fn into(self) -> crate::bindings::VipsFailOn {
        match self {
            VipsFailOn::None => crate::bindings::VipsFailOn_VIPS_FAIL_ON_NONE,
            VipsFailOn::Truncated => crate::bindings::VipsFailOn_VIPS_FAIL_ON_TRUNCATED,
            VipsFailOn::Error => crate::bindings::VipsFailOn_VIPS_FAIL_ON_ERROR,
            VipsFailOn::Warning => crate::bindings::VipsFailOn_VIPS_FAIL_ON_WARNING,
            VipsFailOn::Last => crate::bindings::VipsFailOn_VIPS_FAIL_ON_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsSubsample {
    #[default]
    Auto,
    On,
    Off,
    Last
}

impl Into<crate::bindings::VipsForeignSubsample> for VipsSubsample {
    fn into(self) -> crate::bindings::VipsForeignSubsample {
        match self {
            VipsSubsample::Auto => crate::bindings::VipsForeignSubsample_VIPS_FOREIGN_SUBSAMPLE_AUTO,
            VipsSubsample::On => crate::bindings::VipsForeignSubsample_VIPS_FOREIGN_SUBSAMPLE_ON,
            VipsSubsample::Off => crate::bindings::VipsForeignSubsample_VIPS_FOREIGN_SUBSAMPLE_OFF,
            VipsSubsample::Last => crate::bindings::VipsForeignSubsample_VIPS_FOREIGN_SUBSAMPLE_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsPngFilter {
    #[default]
    None,
    Sub,
    Up,
    Average,
    Paeth,
    All
}

impl Into<crate::bindings::VipsForeignPngFilter> for VipsPngFilter {
    fn into(self) -> crate::bindings::VipsForeignPngFilter {
        match self {
            VipsPngFilter::None => crate::bindings::VipsForeignPngFilter_VIPS_FOREIGN_PNG_FILTER_NONE,
            VipsPngFilter::Sub => crate::bindings::VipsForeignPngFilter_VIPS_FOREIGN_PNG_FILTER_SUB,
            VipsPngFilter::Up => crate::bindings::VipsForeignPngFilter_VIPS_FOREIGN_PNG_FILTER_UP,
            VipsPngFilter::Average => crate::bindings::VipsForeignPngFilter_VIPS_FOREIGN_PNG_FILTER_AVG,
            VipsPngFilter::Paeth => crate::bindings::VipsForeignPngFilter_VIPS_FOREIGN_PNG_FILTER_PAETH,
            VipsPngFilter::All => crate::bindings::VipsForeignPngFilter_VIPS_FOREIGN_PNG_FILTER_ALL,
        }
    }
}

// ---

#[bitmask(u8)]
pub enum VipsKeep {
    None = 0,
    Exif = 1,
    XMP = 2,
    IPTC = 4,
    ICC = 8,
    Other = 16,
    All = 31
}

// ---

#[derive(Debug, Default)]
pub enum VipsWebpPreset {
    #[default]
    Default,
    Picture,
    Photo,
    Drawing,
    Icon,
    Text,
    Last
}

impl Into<crate::bindings::VipsForeignWebpPreset> for VipsWebpPreset {
    fn into(self) -> crate::bindings::VipsForeignWebpPreset {
        match self {
            VipsWebpPreset::Default => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_DEFAULT,
            VipsWebpPreset::Picture => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_PICTURE,
            VipsWebpPreset::Photo => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_PHOTO,
            VipsWebpPreset::Drawing => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_DRAWING,
            VipsWebpPreset::Icon => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_ICON,
            VipsWebpPreset::Text => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_TEXT,
            VipsWebpPreset::Last => crate::bindings::VipsForeignWebpPreset_VIPS_FOREIGN_WEBP_PRESET_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsHeifCompression {
    #[default]
    Last,
    HEVC,
    AVC,
    JPEG,
    AV1,
}

impl Into<crate::bindings::VipsForeignHeifCompression> for VipsHeifCompression {
    fn into(self) -> crate::bindings::VipsForeignHeifCompression {
        match self {
            VipsHeifCompression::HEVC => crate::bindings::VipsForeignHeifCompression_VIPS_FOREIGN_HEIF_COMPRESSION_HEVC,
            VipsHeifCompression::AVC => crate::bindings::VipsForeignHeifCompression_VIPS_FOREIGN_HEIF_COMPRESSION_AVC,
            VipsHeifCompression::JPEG => crate::bindings::VipsForeignHeifCompression_VIPS_FOREIGN_HEIF_COMPRESSION_JPEG,
            VipsHeifCompression::AV1 => crate::bindings::VipsForeignHeifCompression_VIPS_FOREIGN_HEIF_COMPRESSION_AV1,
            VipsHeifCompression::Last => crate::bindings::VipsForeignHeifCompression_VIPS_FOREIGN_HEIF_COMPRESSION_LAST,
        }
    }
}

// ---

#[derive(Debug, Default)]
pub enum VipsHeifEncoder {
    #[default]
    Auto,
    AOM,
    RAV1E,
    SVT,
    X265,
    Last
}

impl Into<crate::bindings::VipsForeignHeifEncoder> for VipsHeifEncoder {
    fn into(self) -> crate::bindings::VipsForeignHeifEncoder {
        match self {
            VipsHeifEncoder::Auto => crate::bindings::VipsForeignHeifEncoder_VIPS_FOREIGN_HEIF_ENCODER_AUTO,
            VipsHeifEncoder::AOM => crate::bindings::VipsForeignHeifEncoder_VIPS_FOREIGN_HEIF_ENCODER_AOM,
            VipsHeifEncoder::RAV1E => crate::bindings::VipsForeignHeifEncoder_VIPS_FOREIGN_HEIF_ENCODER_RAV1E,
            VipsHeifEncoder::SVT => crate::bindings::VipsForeignHeifEncoder_VIPS_FOREIGN_HEIF_ENCODER_SVT,
            VipsHeifEncoder::X265 => crate::bindings::VipsForeignHeifEncoder_VIPS_FOREIGN_HEIF_ENCODER_X265,
            VipsHeifEncoder::Last => crate::bindings::VipsForeignHeifEncoder_VIPS_FOREIGN_HEIF_ENCODER_LAST,
        }
    }
}