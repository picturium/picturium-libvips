use std::os::raw::{c_int};
use crate::{Vips, VipsHeifCompression, VipsHeifEncoder, VipsImage, VipsKeep, VipsPngFilter, VipsSubsample, VipsWebpPreset, NULL};
use crate::bindings::{vips_heifsave, vips_jpegsave, vips_pngsave, vips_webpsave};
use crate::result::{Error, Result};
use crate::utils::{c_string, VipsArrayDouble};

pub trait VipsSaving {
    fn save_jpeg(&self, filename: &str, options: Option<JpegSaveOptions>) -> Result<()>;
    fn save_png(&self, filename: &str, options: Option<PngSaveOptions>) -> Result<()>;
    fn save_webp(&self, filename: &str, options: Option<WebpSaveOptions>) -> Result<()>;
    fn save_heif(&self, filename: &str, options: Option<HeifSaveOptions>) -> Result<()>;
}

impl VipsSaving for VipsImage {
    fn save_jpeg(&self, filename: &str, options: Option<JpegSaveOptions>) -> Result<()> {
        let result = match options {
            Some(options) => unsafe {
                vips_jpegsave(
                    self.0, c_string(filename)?.as_ptr(),
                    c_string("Q")?.as_ptr(), options.q,
                    c_string("optimize-coding")?.as_ptr(), options.optimize_coding as c_int,
                    c_string("interlace")?.as_ptr(), options.interlace as c_int,
                    c_string("subsample-mode")?.as_ptr(), options.subsample_mode,
                    c_string("trellis-quant")?.as_ptr(), options.trellis_quant as c_int,
                    c_string("overshoot-deringing")?.as_ptr(), options.overshoot_deringing as c_int,
                    c_string("optimize-scans")?.as_ptr(), options.optimize_scans as c_int,
                    c_string("quant-table")?.as_ptr(), options.quant_table,
                    c_string("restart-interval")?.as_ptr(), options.restart_interval,
                    c_string("keep")?.as_ptr(), options.keep,
                    c_string("background")?.as_ptr(), VipsArrayDouble::from(options.background).0,
                    c_string("page-height")?.as_ptr(), options.page_height,
                    c_string("profile")?.as_ptr(), c_string(&options.profile)?.as_ptr(),
                    NULL
                )
            },
            None => unsafe { vips_jpegsave(self.0, c_string(filename)?.as_ptr(), NULL) }
        };

        if result != 0 {
            return Err(Error::ImageSaveError(Vips::get_error()))
        }

        Ok(())
    }

    fn save_png(&self, filename: &str, options: Option<PngSaveOptions>) -> Result<()> {
        let result = match options {
            Some(options) => unsafe {
                vips_pngsave(
                    self.0, c_string(filename)?.as_ptr(),
                    c_string("compression")?.as_ptr(), options.compression,
                    c_string("interlace")?.as_ptr(), options.interlace as c_int,
                    c_string("filter")?.as_ptr(), options.filter,
                    c_string("palette")?.as_ptr(), options.palette as c_int,
                    c_string("Q")?.as_ptr(), options.q,
                    c_string("dither")?.as_ptr(), options.dither,
                    c_string("bitdepth")?.as_ptr(), options.bitdepth,
                    c_string("effort")?.as_ptr(), options.effort,
                    c_string("keep")?.as_ptr(), options.keep,
                    c_string("background")?.as_ptr(), VipsArrayDouble::from(options.background).0,
                    c_string("page-height")?.as_ptr(), options.page_height,
                    c_string("profile")?.as_ptr(), c_string(&options.profile)?.as_ptr(),
                    NULL
                )
            },
            None => unsafe { vips_pngsave(self.0, c_string(filename)?.as_ptr(), NULL) }
        };

        if result != 0 {
            return Err(Error::ImageSaveError(Vips::get_error()))
        }

        Ok(())
    }

    fn save_webp(&self, filename: &str, options: Option<WebpSaveOptions>) -> Result<()> {
        let result = match options {
            Some(options) => unsafe {
                vips_webpsave(
                    self.0, c_string(filename)?.as_ptr(),
                    c_string("Q")?.as_ptr(), options.q,
                    c_string("lossless")?.as_ptr(), options.lossless as c_int,
                    c_string("preset")?.as_ptr(), options.preset,
                    c_string("smart-subsample")?.as_ptr(), options.smart_subsample as c_int,
                    c_string("near-lossless")?.as_ptr(), options.near_lossless as c_int,
                    c_string("alpha-q")?.as_ptr(), options.alpha_q,
                    c_string("min-size")?.as_ptr(), options.min_size as c_int,
                    c_string("kmin")?.as_ptr(), options.kmin,
                    c_string("kmax")?.as_ptr(), options.kmax,
                    c_string("effort")?.as_ptr(), options.effort,
                    c_string("keep")?.as_ptr(), options.keep,
                    c_string("background")?.as_ptr(), VipsArrayDouble::from(options.background).0,
                    c_string("page-height")?.as_ptr(), options.page_height,
                    c_string("profile")?.as_ptr(), c_string(&options.profile)?.as_ptr(),

                    NULL
                )
            },
            None => unsafe { vips_webpsave(self.0, c_string(filename)?.as_ptr(), NULL) }
        };

        if result != 0 {
            return Err(Error::ImageSaveError(Vips::get_error()))
        }

        Ok(())
    }

    fn save_heif(&self, filename: &str, options: Option<HeifSaveOptions>) -> Result<()> {
        let result = match options {
            Some(options) => unsafe {
                vips_heifsave(
                    self.0, c_string(filename)?.as_ptr(),
                    c_string("Q")?.as_ptr(), options.q,
                    c_string("bitdepth")?.as_ptr(), options.bitdepth,
                    c_string("lossless")?.as_ptr(), options.lossless as c_int,
                    c_string("compression")?.as_ptr(), options.compression,
                    c_string("effort")?.as_ptr(), options.effort,
                    c_string("subsample-mode")?.as_ptr(), options.subsample_mode,
                    c_string("encoder")?.as_ptr(), options.encoder,
                    c_string("keep")?.as_ptr(), options.keep,
                    c_string("background")?.as_ptr(), VipsArrayDouble::from(options.background).0,
                    c_string("page-height")?.as_ptr(), options.page_height,
                    c_string("profile")?.as_ptr(), c_string(&options.profile)?.as_ptr(),
                    NULL
                )
            },
            None => unsafe { vips_heifsave(self.0, c_string(filename)?.as_ptr(), NULL) }
        };

        if result != 0 {
            return Err(Error::ImageSaveError(Vips::get_error()))
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct JpegSaveOptions<'a> {
    pub q: i32,
    pub optimize_coding: bool,
    pub interlace: bool,
    pub subsample_mode: VipsSubsample,
    pub trellis_quant: bool,
    pub overshoot_deringing: bool,
    pub optimize_scans: bool,
    pub quant_table: i32,
    pub restart_interval: i32,
    pub keep: VipsKeep,
    pub background: &'a [f64],
    pub page_height: i32,
    pub profile: String
}

impl<'a> Default for JpegSaveOptions<'a> {
    fn default() -> Self {
        Self {
            q: 75,
            optimize_coding: false,
            interlace: false,
            subsample_mode: VipsSubsample::default(),
            trellis_quant: false,
            overshoot_deringing: false,
            optimize_scans: false,
            quant_table: 0,
            restart_interval: 0,
            keep: VipsKeep::All,
            background: &[],
            page_height: 0,
            profile: "srgb".into()
        }
    }
}

#[derive(Debug)]
pub struct PngSaveOptions<'a> {
    pub compression: i32,
    pub interlace: bool,
    pub filter: VipsPngFilter,
    pub palette: bool,
    pub q: i32,
    pub dither: f64,
    pub bitdepth: i32,
    pub effort: i32,
    pub keep: VipsKeep,
    pub background: &'a [f64],
    pub page_height: i32,
    pub profile: String
}

impl<'a> Default for PngSaveOptions<'a> {
    fn default() -> Self {
        Self {
            compression: 6,
            interlace: false,
            filter: VipsPngFilter::default(),
            palette: false,
            q: 100,
            dither: 1.0,
            bitdepth: 8,
            effort: 7,
            keep: VipsKeep::All,
            background: &[],
            page_height: 0,
            profile: "srgb".into()
        }
    }
}

#[derive(Debug)]
pub struct WebpSaveOptions<'a> {
    pub q: i32,
    pub lossless: bool,
    pub preset: VipsWebpPreset,
    pub smart_subsample: bool,
    pub near_lossless: bool,
    pub alpha_q: i32,
    pub min_size: bool,
    pub kmin: i32,
    pub kmax: i32,
    pub effort: i32,
    pub keep: VipsKeep,
    pub background: &'a [f64],
    pub page_height: i32,
    pub profile: String
}

impl<'a> Default for WebpSaveOptions<'a> {
    fn default() -> Self {
        Self {
            q: 75,
            lossless: false,
            preset: VipsWebpPreset::default(),
            smart_subsample: false,
            near_lossless: false,
            alpha_q: 100,
            min_size: false,
            kmin: 2147483646,
            kmax: 2147483647,
            effort: 4,
            keep: VipsKeep::All,
            background: &[],
            page_height: 0,
            profile: "srgb".into()
        }
    }
}

#[derive(Debug)]
pub struct HeifSaveOptions<'a> {
    pub q: i32,
    pub bitdepth: i32,
    pub lossless: bool,
    pub compression: VipsHeifCompression,
    pub effort: i32,
    pub subsample_mode: VipsSubsample,
    pub encoder: VipsHeifEncoder,
    pub keep: VipsKeep,
    pub background: &'a [f64],
    pub page_height: i32,
    pub profile: String
}

impl<'a> Default for HeifSaveOptions<'a> {
    fn default() -> Self {
        Self {
            q: 50,
            bitdepth: 8,
            lossless: false,
            compression: VipsHeifCompression::default(),
            effort: 4,
            subsample_mode: VipsSubsample::default(),
            encoder: VipsHeifEncoder::default(),
            keep: VipsKeep::All,
            background: &[],
            page_height: 0,
            profile: "srgb".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vips::Vips;

    #[test]
    fn it_can_save_image_to_jpeg() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        image.unwrap().save_jpeg("data/output/example.jpg", JpegSaveOptions {
            background: &[128.0, 128.0, 256.0],
            ..Default::default()
        }.into()).expect("Failed to save image to JPEG");
    }

    #[test]
    fn it_can_save_image_to_png() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        image.unwrap().save_png("data/output/example.png", Some(PngSaveOptions {
            background: &[128.0, 128.0, 128.0],
            ..Default::default()
        })).expect("Failed to save image to PNG");
    }

    #[test]
    fn it_can_save_image_to_webp() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        image.unwrap().save_webp("data/output/example.webp", Some(WebpSaveOptions {
           ..Default::default()
        })).expect("Failed to save image to WebP");
    }

    #[test]
    fn it_can_save_image_to_heif() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        image.unwrap().save_heif("data/output/example.avif", None).expect("Failed to save image to AVIF");
    }
}