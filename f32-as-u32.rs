fn main() {
    let a: f32 = 42.42;
    let franktype:u32 = unsafe {
       std::mem::transmute(a) 
    };
    println!("{}", franktype);
    println!("{:032b}", franktype);
    let b: f32 = unsafe {
        std::mem::transmute(franktype)
    };
    println!("{}", b);
}