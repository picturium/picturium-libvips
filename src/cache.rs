#[derive(Debug)]
pub struct Cache {
    pub(crate) capacity: i32,
    pub(crate) memory_capacity: usize,
    pub(crate) files_capacity: i32
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            capacity: 100, // operations
            memory_capacity: 100 * 1024 * 1024, // 100 MB
            files_capacity: 100 // files
        }
    }
}

impl Cache {
    pub fn new(capacity: i32, memory_capacity: usize, files_capacity: i32) -> Self {
        Cache {
            capacity,
            memory_capacity,
            files_capacity
        }
    }
    
    pub fn disabled() -> Self {
        Cache::new(0, 0, 0)
    }
}