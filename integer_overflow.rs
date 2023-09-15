fn main() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1;
        print!("..{:016b}..{}..", i,i);
        if i % 10 == 0 {
            print!{"\n"}
        }
    };
}