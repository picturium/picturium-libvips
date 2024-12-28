use crate::enums::{VipsAccess, VipsIntent, VipsInterpretation, VipsPCS};

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