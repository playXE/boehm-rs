use boehm_rs::global_alloc::GcAlloc;
use boehm_rs::{gc_collect, gc_enable, gc_init};

#[global_allocator]
static A: GcAlloc = GcAlloc;

use std::thread::*;

fn main() {
    gc_enable();
    gc_init();

    spawn(|| {
        println!("{}", String::from("Hello,world!"));
    })
    .join()
    .unwrap();

    gc_collect();
}
