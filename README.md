# boehm-rs

Rust interface to BoehmGC

# Examples

```rust

use boehm_rs::global_alloc::GcAlloc;
use boehm_rs::{gc_init,gc_enable};

#[global_allocator]
static A: GcAlloc = GcAlloc;

fn main() {
    gc_enable();
    gc_init();
    let string = String::from("Hello,world!);

    println!("{}",string);
}
```


```

```