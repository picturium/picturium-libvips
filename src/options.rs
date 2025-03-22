use crate::enums::{VipsAccess, VipsIntent, VipsInterpretation, VipsPCS, VipsForeignFlags};
use crate::VipsFailOn;

#[derive(Debug)]
pub struct FromFileOptions {
    pub access: VipsAccess,
    pub memory: bool
}

impl Default for FromFileOptions {
    fn default() -> Self {
        Self {
            access: VipsAccess::default(),
            memory: true
        }
    }
}

#[derive(Debug)]
pub struct FromSvgOptions {
    pub dpi: f64,
    pub scale: f64,
    pub unlimited: bool,
    pub flags: VipsForeignFlags,
    pub memory: bool,
    pub access: VipsAccess,
    pub fail_on: VipsFailOn,
    pub revalidate: bool
}

impl Default for FromSvgOptions {
    fn default() -> Self {
        Self {
            dpi: 72.0,
            scale: 1.0,
            unlimited: false,
            flags: VipsForeignFlags::default(),
            memory: false,
            access: VipsAccess::default(),
            fail_on: VipsFailOn::default(),
            revalidate: false,
        }
    }
}

#[derive(Debug)]
pub struct IccTransformOptions {
    pub pcs: VipsPCS,
    pub intent: VipsIntent,
    pub black_point_compensation: bool,
    pub embedded: bool,
    pub input_profile: String,
    pub depth: i32
}

impl Default for IccTransformOptions {
    fn default() -> Self {
        Self {
            pcs: VipsPCS::default(),
            intent: VipsIntent::default(),
            black_point_compensation: false,
            embedded: false,
            input_profile: "sRGB".into(),
            depth: 8
        }
    }
}

#[derive(Debug)]
pub struct Composite2Options {
    pub compositing_space: VipsInterpretation,
    pub premultiplied: bool,
    pub x: i32,
    pub y: i32
}

impl Default for Composite2Options {
    fn default() -> Self {
        Self {
            compositing_space: VipsInterpretation::default(),
            premultiplied: false,
            x: 0,
            y: 0
        }
    }
}