use boehm_rs::*;
use boehm_rs::global_alloc::GcAlloc;

#[global_allocator]
static A: GcAlloc = GcAlloc;

fn main() {
    gc_enable();
    gc_init();

    gc_collect();
}