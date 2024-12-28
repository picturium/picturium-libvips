use std::ffi::CString;
use std::os::raw::c_int;
use crate::bindings::{vips_area_unref, vips_array_double_new, VipsArea};
use crate::result::{Result, Error};

pub fn c_string(string: &str) -> Result<CString> {
    CString::new(string).map_err(|_| Error::CError("Could not create C string from Rust string"))
}

// ---

pub struct VipsArrayDouble(pub(crate) *mut crate::bindings::VipsArrayDouble);

impl From<&[f64]> for VipsArrayDouble {
    fn from(array: &[f64]) -> Self {
        VipsArrayDouble(unsafe { vips_array_double_new(array.as_ptr(), array.len() as c_int) })
    }
}

impl Drop for VipsArrayDouble {
    fn drop(&mut self) {
        unsafe { vips_area_unref(self.0 as *mut VipsArea) }
    }
}
