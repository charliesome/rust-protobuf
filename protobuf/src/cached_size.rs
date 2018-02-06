use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use types::ProtobufType;

/// Cached size field used in generated code.
/// It is always equal to itself to simplify generated code.
/// (Generated code can use `#[derive(Eq)]`).
#[derive(Debug, Default)]
pub struct CachedSize {
    size: AtomicUsize,
}

impl CachedSize {
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed) as u32
    }

    pub fn set(&self, size: u32) {
        self.size.store(size as usize, Ordering::Relaxed)
    }
}

impl Clone for CachedSize {
    fn clone(&self) -> CachedSize {
        CachedSize {
            size: AtomicUsize::new(self.size.load(Ordering::Relaxed))
        }
    }
}

impl PartialEq<CachedSize> for CachedSize {
    fn eq(&self, _other: &CachedSize) -> bool {
        true
    }
}

impl Eq for CachedSize {}

pub struct SizeCache {
    sizes: HashMap<*const (), usize>,
}

impl SizeCache {
    pub fn new() -> Self {
        SizeCache { sizes: HashMap::new() }
    }

    pub fn size_of<T: ProtobufType>(&mut self, val: &T::Value) -> usize {
        let key = val as *const T::Value as *const ();

        let cached_size = self.sizes
            .entry(key)
            .or_insert_with(|| T::compute_size(val) as usize);

        *cached_size
    }
}
