fn main () {
    let a:u8 = 0b11111111; 
    let base: u32 = 0b0_0111_1110_0000_0000_0000_0000_0000_000;
    let move_into_u32: u32 = (a as u32) << 15;
    let into_u32: u32 = base | move_into_u32;
    let into_f32: f32 = f32::from_bits(into_u32);
    println!("{}", 2_f32 * (into_f32 - 0.5));
}