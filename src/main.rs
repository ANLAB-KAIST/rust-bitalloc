extern crate bitalloc;

fn main() {
    let mut array = [0u64; 128];
    let mut a = bitalloc::Bitalloc::wrap(64*16, &mut array).expect("Cannot wrap given buffer");
    for _ in 0..320 {
        println!("allocated {}", a.alloc(64*2 + 13));
    }
}
