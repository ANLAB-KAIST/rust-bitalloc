extern crate bitalloc;
use bitalloc::Bitalloc;
fn main() {
    let mut array = [0u64; 128];
    let mut array2 = [0u8; 128*8];
    let slice =&mut  array;
    let slice2 =&mut  array2;
    for _ in 0..32 {
        println!("allocated {} {}", slice.alloc(64*2 + 13), slice2.alloc(64*2 + 13));
    }
}
