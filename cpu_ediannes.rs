fn main() {
    let b_edian: [u8;4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let l_edian: [u8;4] = [0xDD, 0xCC, 0xBB, 0xAA];
    let a: i32 = unsafe {std::mem::transmute(b_edian)};
    let b: i32 = unsafe {std::mem::transmute(l_edian)};
    println!("{} vs {}", a, b);
    println!("{:032b} vs {:032b}", a, b);
}