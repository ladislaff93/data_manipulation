// 32bit float:    0    | 00000000 | 00000000 00000000 0000000
//              Sign Bit| Exponent | Mantissa = 1.(Fractional Bits)
//  (-1**Sign Bit) * Mantissa * (2 **(Exponent-127))
// (1 10000000 11000000 00000000 0000000) => (-1**1) * 1.75 * (2**(128-127)) => -3.5
const RADIX: f32 = 2.0;
const BIAS: u32 = 127;
fn to_parts(num: f32) -> (u32, u32, u32){
    let bits_num = num.to_bits();

    let sign_bit = bits_num >> 31;
    let exponent = (bits_num >> 23) & 0xFF; //0b_0_1111_1111
    let fractions = bits_num & 0x7FFFFF; //0b_0_00000000_11111111_11111111_1111111 
    (sign_bit, exponent, fractions)
}
fn decode(sign: u32, exponent: u32, fractions: u32) -> (f32, f32, f32) {
    let signed_1: f32 = (-1_f32).powf(sign as f32);
    let exponent: f32 = RADIX.powf((exponent - BIAS) as f32);
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fractions & mask;
        if one_at_bit_i != 0 {
            let frac = 2_f32.powf(i as f32 -23.0);
            mantissa += frac
        }
    } 
    (signed_1, exponent, mantissa)
}
fn from_parts(sign:f32, exponent:f32, mantissa:f32) -> f32 {
    sign * exponent * mantissa
}

fn main() {
    let n: f32 = 42.42;
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let float_n = from_parts(sign_, exp_, mant);
    println!("{} -> {}", n, float_n);
    println!("field    |  as bits | as real number");
    println!("sign     |        {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant); 
}