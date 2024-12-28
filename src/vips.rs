use std::ffi::CStr;
use crate::bindings::{vips_cache_get_max, vips_cache_get_max_files, vips_cache_get_max_mem, vips_cache_get_size, vips_cache_set_max, vips_cache_set_max_files, vips_cache_set_max_mem, vips_concurrency_get, vips_concurrency_set, vips_error_buffer, vips_error_clear, vips_get_disc_threshold, vips_init, vips_leak_set, vips_shutdown, vips_thread_shutdown, vips_tracked_get_allocs, vips_tracked_get_files, vips_tracked_get_mem, vips_tracked_get_mem_highwater};
use crate::cache::Cache;
use crate::result::Error;
use crate::utils::c_string;

pub struct Vips;

impl Vips {
    pub fn new(name: &str) -> crate::result::Result<Self> {
        let name = c_string(name)?;

        match unsafe { vips_init(name.as_ptr()) } {
            0 => Ok(Vips),
            _ => Err(Error::UnknownError("Could not initialize libvips library")),
        }
    }

    pub fn check_leaks(&self) -> &Self {
        unsafe { vips_leak_set(true as _); }
        self
    }

    pub fn cache(&self, cache: Cache) -> &Self {
        unsafe {
            vips_cache_set_max(cache.capacity);
            vips_cache_set_max_mem(cache.memory_capacity);
            vips_cache_set_max_files(cache.files_capacity);
        }

        self
    }

    pub fn concurrency(&self, concurrency: i32) -> &Self {
        unsafe { vips_concurrency_set(concurrency); }
        self
    }

    pub fn shutdown(&self) {
        unsafe { vips_shutdown(); }
    }

    pub fn thread_shutdown() {
        unsafe { vips_thread_shutdown(); }
    }
    
    // 

    pub fn get_disk_threshold() -> u64 {
        unsafe { vips_get_disc_threshold() }
    }

    pub fn get_memory_usage() -> usize {
        unsafe { vips_tracked_get_mem() }
    }

    pub fn get_peak_memory_usage() -> usize {
        unsafe { vips_tracked_get_mem_highwater() }
    }

    pub fn get_active_allocations() -> i32 {
        unsafe { vips_tracked_get_allocs() }
    }

    pub fn get_open_files() -> i32 {
        unsafe { vips_tracked_get_files() }
    }

    pub fn get_cache_size() -> i32 {
        unsafe { vips_cache_get_size() }
    }

    pub fn get_cache_capacity() -> i32 {
        unsafe { vips_cache_get_max() }
    }

    pub fn get_cache_memory_capacity() -> usize {
        unsafe { vips_cache_get_max_mem() }
    }

    pub fn get_cache_files_capacity() -> i32 {
        unsafe { vips_cache_get_max_files() }
    }

    pub fn get_concurrency_threads() -> i32 {
        unsafe { vips_concurrency_get() }
    }

    pub fn get_error() -> String {
        let error_buffer_ptr: *const ::std::os::raw::c_char = unsafe { vips_error_buffer() };
        let c_str = unsafe { CStr::from_ptr(error_buffer_ptr) };
        let error_message = c_str.to_string_lossy().into_owned();
        unsafe { vips_error_clear(); }
        error_message
    }
}

impl Drop for Vips {
    fn drop(&mut self) {
        self.shutdown()
    }
}
