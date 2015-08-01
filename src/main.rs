extern crate bitalloc;
use bitalloc::Bitalloc;
fn main() {
    let mut array1 = [0u8; 1024];
    let mut array2 = [0u16; 512];
    let mut array3 = [0u32; 256];
    let mut array4 = [0u64; 128];
    let slice1 =&mut  array1;
    let slice2 =&mut  array2;
    let slice3 =&mut  array3;
    let slice4 =&mut  array4;
    for _ in 0..32 {
        let ret1 = slice1.alloc(64*2 + 13);
        let ret2 = slice2.alloc(64*2 + 13);
        let ret3 = slice3.alloc(64*2 + 13);
        let ret4 = slice4.alloc(64*2 + 13);
        assert!(ret1 == ret2 && ret2 == ret3 && ret3 == ret4);
    }
    for x in 4..12 {
        slice1.free((64*2 + 17) * x, 64 + 37);
        slice2.free((64*2 + 17) * x, 64 + 37);
        slice3.free((64*2 + 17) * x, 64 + 37);
        slice4.free((64*2 + 17) * x, 64 + 37);
    }
    for _ in 0..32 {
        let ret1 = slice1.alloc(64*2 + 13);
        let ret2 = slice2.alloc(64*2 + 13);
        let ret3 = slice3.alloc(64*2 + 13);
        let ret4 = slice4.alloc(64*2 + 13);
        assert!(ret1 == ret2 && ret2 == ret3 && ret3 == ret4);
    }
    println!("Finished");
}
