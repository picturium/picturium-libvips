use std::os::raw::{c_int};
use std::ptr::null_mut;
use crate::bindings::{vips_autorot, vips_composite2, vips_icc_transform, vips_rot};
use crate::enums::{VipsAngle, VipsBlendMode};
use crate::image::{VipsImage, NULL};
use crate::options::{Composite2Options, IccTransformOptions};
use crate::result::{Error, Result};
use crate::utils::c_string;
use crate::vips::Vips;

pub trait VipsOperations {
    fn icc_transform(self, output_profile: &str, options: Option<IccTransformOptions>) -> Result<Self> where Self: Sized;
    fn autorotate(self) -> Result<Self> where Self: Sized;
    fn rotate(self, angle: VipsAngle) -> Result<Self> where Self: Sized;
    fn background_color(self, color: &[f64]) -> Result<Self> where Self: Sized;
    fn composite2(self, overlay: &VipsImage, mode: VipsBlendMode, options: Option<Composite2Options>) -> Result<Self> where Self: Sized;
}

impl VipsOperations for VipsImage {
    fn icc_transform(self, output_profile: &str, options: Option<IccTransformOptions>) -> Result<Self> {
        let mut output_image: *mut crate::bindings::VipsImage = null_mut();

        let result = match options {
            Some(options) => unsafe {
                vips_icc_transform(
                    self.0, &mut output_image, c_string(output_profile)?.as_ptr(),
                    c_string("pcs")?.as_ptr(), options.pcs,
                    c_string("intent")?.as_ptr(), options.intent,
                    c_string("black-point-compensation")?.as_ptr(), options.black_point_compensation as c_int,
                    c_string("embedded")?.as_ptr(), options.embedded as c_int,
                    c_string("input-profile")?.as_ptr(), c_string(&options.input_profile)?.as_ptr(),
                    c_string("depth")?.as_ptr(), options.depth,
                    NULL
                )
            },
            None => unsafe { vips_icc_transform(self.0, &mut output_image, c_string(output_profile)?.as_ptr(), NULL) }
        };

        if result != 0 || output_image.is_null() {
            return Err(Error::ImageOperationError(Vips::get_error()));
        }

        let mut output_image = VipsImage(output_image, None);
        output_image.keepalive(self);

        Ok(output_image)
    }

    fn autorotate(self) -> Result<Self> {
        let mut output_image: *mut crate::bindings::VipsImage = null_mut();

        if unsafe { vips_autorot(self.0, &mut output_image, NULL) != 0 } || output_image.is_null() {
            return Err(Error::ImageOperationError(Vips::get_error()));
        }

        let mut output_image = VipsImage(output_image, None);
        output_image.keepalive(self);

        Ok(output_image)
    }

    fn rotate(self, angle: VipsAngle) -> Result<Self> {
        let mut output_image: *mut crate::bindings::VipsImage = null_mut();

        if unsafe { vips_rot(self.0, &mut output_image, angle.into(), NULL) != 0 } || output_image.is_null() {
            return Err(Error::ImageOperationError(Vips::get_error()));
        }

        let mut output_image = VipsImage(output_image, None);
        output_image.keepalive(self);

        Ok(output_image)
    }

    fn background_color(self, bands: &[f64]) -> Result<Self> {
        let background_image = VipsImage::new_from_image(&self, bands)?;

        match background_image.composite2(&self, VipsBlendMode::Over, None) {
            Ok(mut image) => {
                image.keepalive(self);
                Ok(image)
            },
            Err(e) => Err(e),
        }
    }

    fn composite2(self, overlay: &VipsImage, mode: VipsBlendMode, options: Option<Composite2Options>) -> Result<Self> {
        let mut output_image: *mut crate::bindings::VipsImage = null_mut();

        let result = match options {
            Some(options) => unsafe {
                vips_composite2(
                    self.0, overlay.0, &mut output_image, mode.into(),
                    c_string("compositing-space")?.as_ptr(), options.compositing_space,
                    c_string("premultiplied")?.as_ptr(), options.premultiplied as c_int,
                    c_string("x")?.as_ptr(), options.x,
                    c_string("y")?.as_ptr(), options.y,
                    NULL
                )
            },
            None => unsafe { vips_composite2(self.0, overlay.0, &mut output_image, mode.into(), NULL) }
        };

        if result != 0 || output_image.is_null() {
            return Err(Error::ImageOperationError(Vips::get_error()));
        }

        let mut output_image = VipsImage(output_image, None);
        output_image.keepalive(self);

        Ok(output_image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vips::Vips;
    use crate::VipsSaving;

    #[test]
    fn it_applies_icc_transform() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let image = image.unwrap();
        let transformed_image = image.icc_transform("p3", IccTransformOptions::default().into());

        if let Err(e) = transformed_image {
            panic!("Transformed image: {e}");
        }

        transformed_image.unwrap().save_jpeg("data/output/icc_transformed.jpg", None).unwrap();
    }

    #[test]
    fn it_automatically_rotates_images() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/autorotate.jpg", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let image = image.unwrap();
        let image = image.autorotate();

        if let Err(e) = image {
            panic!("Rotated image: {e}");
        }

        image.unwrap().save_jpeg("data/output/autorotated.jpg", None).unwrap();
    }

    #[test]
    fn it_rotates_images() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let image = image.unwrap();
        let rotated_image = image.rotate(VipsAngle::Left);

        if let Err(e) = rotated_image {
            panic!("Rotated image: {e}");
        }

        rotated_image.unwrap().save_jpeg("data/output/rotated.jpg", None).unwrap();
    }

    #[test]
    fn it_sets_background_color() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let image = image.unwrap();
        let bands = vec![255.0, 255.0, 255.0];
        let image_with_background = image.background_color(&bands);

        if let Err(e) = image_with_background {
            panic!("Image with background color: {e}");
        }

        let image_with_background = image_with_background.unwrap();

        if let Err(e) = image_with_background.save_png("data/output/background_color.png", None) {
            panic!("Save image with background color: {e}");
        }
    }

    #[test]
    fn it_composites_images() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let image = image.unwrap();

        let overlay = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = overlay {
            panic!("Overlay image: {e}");
        }

        let overlay = overlay.unwrap();
        let composite_image = image.composite2(&overlay, VipsBlendMode::Over, None);

        if let Err(e) = composite_image {
            panic!("Composite image: {e}");
        }

        let composite_image = composite_image.unwrap();

        if let Err(e) = composite_image.save_png("data/output/composite_image.png", None) {
            panic!("Save composite image with background color: {e}");
        }
    }
}