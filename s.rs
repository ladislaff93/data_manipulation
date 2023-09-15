fn main() {
    // let s = 8.0;
    let a: f32 = -44.69;
    let b: f32 = a * (1<<12) as f32;
    let c = a * 2_f32.powf(12_f32);
    println!("before rounding: {}", b);
    println!("before rounding: {}", c);
    let m: i32 = b.round() as i32;
    println!("after rounding: {:032b}", m);
    println!("after rounding: {}", m);
    let mut m = m.abs();
    println!("after abs: {:032b}", m);
    println!("after abs: {}", m);
    let mut m = !m;
    println!("after not: {:032b}", m);
    println!("after not: {}", m);
    m += 1;
    println!("after adding 1: {:032b}", m);
    println!("after adding 1: {}", m);


    // let m = n.round() as i16;
    // if n < 0_f64 {
    //     println!("number before abs: {:016b}", m);
    //     let m = m.abs();
    //     println!("number after  abs: {:016b}", m);
    //     let mut m = !m;
    //     println!("                 : {:016b}", m);
    //     m += 1;
    //     println!("                 : {:016b}", m);
    // }
    // println!("{}", m);
    // println!("{:016b}", m);
    
}