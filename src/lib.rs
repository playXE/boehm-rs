#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;

use self::bindings::*;

///! Rust interface to BoehmGC
///!
///! This crate provides FFI with BoehmGC and Global allocator that uses this GC
///!
pub use core as std;

/// Enable GC
pub fn gc_enable() {
    unsafe {
        GC_enable();
    }
}
/// Initialize GC
pub fn gc_init() {
    unsafe {
        GC_init();
    }
}
/// Disable GC
pub fn gc_disable() {
    unsafe {
        GC_disable();
    }
}
/// Allocates memory, doesn't checks ptr is null
pub unsafe fn gc_malloc(size: usize) -> *mut u8 {
    return GC_malloc(size) as *mut u8;
}

/// Allocates memory,if ptr is null panics
pub fn gc_alloc(size: usize) -> std::ptr::NonNull<u8> {
    let ptr = unsafe { gc_malloc(size) };
    if ptr.is_null() {
        panic!("Failed to allocate memory with size {}", size);
    }

    return std::ptr::NonNull::new(ptr).unwrap();
}

/// Reallocate memory
pub fn gc_realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    return unsafe { GC_realloc(ptr as *mut _, size) as *mut u8 };
}

/// Allocate memory with some size and align
pub fn gc_memalign(size: usize, align: usize) -> *mut u8 {
    return unsafe { GC_memalign(size, align) as *mut u8 };
}
/// Collect garbage
pub fn gc_collect() {
    unsafe {
        GC_gcollect();
    }
}

/// Global allocator that uses BoehmGC for allocating
///
/// # Example of using
/// ```
/// #[global_allocator]
/// static A: GcAlloc = GcAlloc;
///
/// fn main() {
///     gc_enable();
///     gc_init();
///     let string = String::from("Hello,world!");
///     println!("{}",string);
/// }
///
/// ```
pub mod global_alloc {
    use core::alloc::{GlobalAlloc, Layout};

    use super::*;

    extern "C" {
        fn printf(c: *const i8, ...);
    }

    pub struct GcAlloc;
    unsafe impl GlobalAlloc for GcAlloc {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 {
            let ptr = gc_malloc(l.size());
            ptr
        }
        unsafe fn dealloc(&self, _ptr: *mut u8, _: Layout) {
            //do nothing?
        }

        unsafe fn realloc(&self,ptr: *mut u8,_: Layout,new_size: usize) -> *mut u8 {
            gc_realloc(ptr,new_size)
        }
    }

    unsafe impl Send for GcAlloc {}
    unsafe impl Sync for GcAlloc {}
}
