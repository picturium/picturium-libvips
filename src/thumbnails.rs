use std::os::raw::c_int;
use std::ptr::null_mut;
use crate::{Vips, VipsFailOn, VipsImage, VipsIntent, VipsInteresting, VipsSize, NULL};
use crate::bindings::{vips_thumbnail, vips_thumbnail_image};
use crate::result::{Error, Result};
use crate::utils::c_string;

pub trait VipsThumbnails {
    fn thumbnail(filename: &str, width: i32, options: Option<ThumbnailOptions>) -> Result<VipsImage>;
    fn thumbnail_image(self, width: i32, options: Option<ThumbnailOptions>) -> Result<VipsImage>;
}

impl VipsThumbnails for VipsImage {
    fn thumbnail(filename: &str, width: i32, options: Option<ThumbnailOptions>) -> Result<VipsImage> {
        let mut output_image: *mut crate::bindings::VipsImage = null_mut();

        let result = match options {
            Some(options) => unsafe {
                vips_thumbnail(
                    c_string(filename)?.as_ptr(), &mut output_image, width,
                    c_string("height")?.as_ptr(), options.height,
                    c_string("size")?.as_ptr(), options.size,
                    c_string("no-rotate")?.as_ptr(), options.no_rotate as c_int,
                    c_string("crop")?.as_ptr(), options.crop,
                    c_string("linear")?.as_ptr(), options.linear as c_int,
                    c_string("import-profile")?.as_ptr(), c_string(&options.import_profile)?.as_ptr(),
                    c_string("export-profile")?.as_ptr(), c_string(&options.export_profile)?.as_ptr(),
                    c_string("intent")?.as_ptr(), options.intent,
                    c_string("fail_on")?.as_ptr(), options.fail_on,
                    NULL
                )
            },
            None => unsafe { vips_thumbnail(c_string(filename)?.as_ptr(), &mut output_image, width, NULL) }
        };

        if result != 0 || output_image.is_null() {
            return Err(Error::ImageOperationError(Vips::get_error()));
        }

        Ok(VipsImage(output_image, None))
    }

    fn thumbnail_image(self, width: i32, options: Option<ThumbnailOptions>) -> Result<VipsImage> {
        let mut output_image: *mut crate::bindings::VipsImage = null_mut();

        let result = match options {
            Some(options) => unsafe {
                vips_thumbnail_image(
                    self.0, &mut output_image, width,
                    c_string("height")?.as_ptr(), options.height,
                    c_string("size")?.as_ptr(), options.size,
                    c_string("no-rotate")?.as_ptr(), options.no_rotate as c_int,
                    c_string("crop")?.as_ptr(), options.crop,
                    c_string("linear")?.as_ptr(), options.linear as c_int,
                    c_string("import-profile")?.as_ptr(), c_string(&options.import_profile)?.as_ptr(),
                    c_string("export-profile")?.as_ptr(), c_string(&options.export_profile)?.as_ptr(),
                    c_string("intent")?.as_ptr(), options.intent,
                    c_string("fail_on")?.as_ptr(), options.fail_on,
                    NULL
                )
            },
            None => unsafe { vips_thumbnail_image(self.0, &mut output_image, width, NULL) }
        };

        if result != 0 || output_image.is_null() {
            return Err(Error::ImageOperationError(Vips::get_error()));
        }

        let mut output_image = VipsImage(output_image, None);
        output_image.keepalive(self);

        Ok(output_image)
    }
}

#[derive(Debug)]
pub struct ThumbnailOptions {
    pub height: i32,
    pub size: VipsSize,
    pub no_rotate: bool,
    pub crop: VipsInteresting,
    pub linear: bool,
    pub import_profile: String,
    pub export_profile: String,
    pub intent: VipsIntent,
    pub fail_on: VipsFailOn
}

impl Default for ThumbnailOptions {
    fn default() -> Self {
        Self {
            height: 0,
            size: VipsSize::default(),
            no_rotate: false,
            crop: VipsInteresting::default(),
            linear: false,
            import_profile: "sRGB".into(),
            export_profile: "sRGB".into(),
            intent: VipsIntent::default(),
            fail_on: VipsFailOn::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::VipsSaving;
    use super::*;

    #[test]
    fn it_can_generate_thumbnail() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::thumbnail("data/example.jpg", 200, Some(ThumbnailOptions {
            height: 200,
            ..Default::default()
        }));

        if let Err(e) = image {
            panic!("{e}");
        }

        image.unwrap().save_jpeg("data/output/thumbnail.jpg", None).unwrap();
    }

    #[test]
    fn it_can_generate_thumbnail_image() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let mut image = VipsImage::new_from_file("data/example.jpg", None);
        image = VipsImage::thumbnail_image(image.unwrap(), 200, Some(ThumbnailOptions {
            height: 200,
           ..Default::default()
        }));

        if let Err(e) = image {
            panic!("{e}");
        }

        image.unwrap().save_jpeg("data/output/thumbnail.jpg", None).unwrap();
    }
}
