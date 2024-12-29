use std::os::raw::{c_int, c_void};
use crate::bindings::{vips_image_new_from_file, VipsImage as CVipsImage, g_object_unref, vips_image_new_from_image, vips_image_get_width, vips_image_get_height, vips_image_set_kill, vips_image_hasalpha, vips_image_get_n_pages};
use crate::options::{FromFileOptions};
use crate::utils::c_string;
use crate::result::{Error, Result};
use crate::vips::Vips;

#[derive(Debug, Clone)]
pub struct VipsImage(pub *mut CVipsImage, pub(crate) Option<Vec<VipsImage>>);

pub const NULL: *const std::os::raw::c_char = std::ptr::null();

impl VipsImage {
    pub fn new_from_file(filename: &str, options: Option<FromFileOptions>) -> Result<Self> {
        let filename = match c_string(filename) {
            Ok(filename) => filename,
            Err(e) => return Err(e)
        };

        let image = match options {
            Some(options) => unsafe {
                vips_image_new_from_file(
                    filename.as_ptr(),
                    c_string("memory")?.as_ptr(), options.memory as c_int,
                    c_string("access")?.as_ptr(), options.access,
                    NULL
                )
            },
            None => unsafe { vips_image_new_from_file(filename.as_ptr(), NULL) }
        };

        if image.is_null() {
            return Err(Error::ImageLoadError(Vips::get_error()));
        }

        Ok(VipsImage(image, None))
    }

    pub fn new_from_image(image: &VipsImage, bands: &[f64]) -> Result<Self> {
        let image = unsafe { vips_image_new_from_image(image.0, bands.as_ptr(), bands.len() as c_int) };

        if image.is_null() {
            return Err(Error::ImageLoadError(Vips::get_error()));
        }

        Ok(VipsImage(image, None))
    }

    pub fn new_from_self(&self, bands: &[f64]) -> Result<Self> {
        Self::new_from_image(self, bands)
    }

    pub fn get_width(&self) -> i32 {
        unsafe { vips_image_get_width(self.0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { vips_image_get_height(self.0) }
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.get_width(), self.get_height())
    }

    pub fn get_bands(&self) -> i32 {
        (unsafe { *self.0 }).Bands
    }

    pub fn get_page_count(&self) -> i32 {
        unsafe { vips_image_get_n_pages(self.0) }
    }

    pub fn is_transparent(&self) -> bool {
        unsafe { vips_image_hasalpha(self.0) == 1 }
    }

    pub fn kill(&self) {
        unsafe { vips_image_set_kill(self.0, 1) }
    }

    pub(crate) fn keepalive(&mut self, image: VipsImage) {
        if let None = self.1 {
            self.1 = Some(Vec::new());
        }
        
        self.1.as_mut().unwrap().push(image);
    }
    
    pub(crate) fn cleanup(&self) {
        self.kill();

        if !self.0.is_null() {
            unsafe { g_object_unref(self.0 as *mut c_void); }
        }
    }
}

impl Drop for VipsImage {
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::VipsAccess;
    use crate::vips::Vips;
    use super::*;

    #[test]
    fn it_creates_a_new_image_from_a_file() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", FromFileOptions {
            access: VipsAccess::Last,
            memory: true,
        }.into());

        if let Err(e) = image {
            panic!("{e}");
        }
    }

    #[test]
    fn it_creates_a_new_image_from_image() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let new_image = image.unwrap().new_from_self(&[255.0, 255.0, 255.0]);

        if let Err(e) = new_image {
            panic!("New image: {e}");
        }
    }

    #[test]
    fn it_returns_image_dimensions() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Original image: {e}");
        }

        let image = image.unwrap();

        assert_eq!(image.get_width(), 4000);
        assert_eq!(image.get_height(), 5328);
        assert_eq!(image.get_dimensions(), (4000, 5328));
    }

    #[test]
    fn it_returns_number_of_bands() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Opaque image: {e}");
        }

        let image = image.unwrap();
        assert_eq!(image.get_bands(), 3);

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Transparent image: {e}");
        }

        let image = image.unwrap();
        assert_eq!(image.get_bands(), 4);
    }

    #[test]
    fn it_returns_transparency_information() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let image = VipsImage::new_from_file("data/example.jpg", None);

        if let Err(e) = image {
            panic!("Opaque image: {e}");
        }

        let image = image.unwrap();
        assert_eq!(image.is_transparent(), false);

        let image = VipsImage::new_from_file("data/transparent.png", None);

        if let Err(e) = image {
            panic!("Transparent image: {e}");
        }

        let image = image.unwrap();
        assert_eq!(image.is_transparent(), true);
    }

    #[test]
    fn it_returns_number_of_pages() {
        let vips = Vips::new("picturium").unwrap();
        vips.check_leaks();

        let document = VipsImage::new_from_file("data/document.pdf", None);

        if let Err(e) = document {
            panic!("PDF document: {e}");
        }

        let document = document.unwrap();
        assert_eq!(document.get_page_count(), 2);
    }
}