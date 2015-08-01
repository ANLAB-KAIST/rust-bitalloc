extern crate bitalloc;
use bitalloc::Bitalloc;
fn main() {
    let mut array = [0u64; 128];
    let slice =&mut  array;
    for _ in 0..320 {
        println!("allocated {}", slice.alloc(64*2 + 13));
    }
}
