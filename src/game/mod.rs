use num_bigint::BigUint;
use std::mem;

mod early_vm;

pub fn main() {
    // println!("{:032b}", unsafe {
    //     mem::transmute::<_, u32>(f32::INFINITY)
    // });
    // println!("{:032b}", unsafe {
    //     mem::transmute::<_, u32>(f32::NEG_INFINITY)
    // });
    // println!("{:032b}", unsafe {
    //     mem::transmute::<_, u32>(0b01111111100000000000000000000000 as f32)
    // });

    let x = (0x1000) as u32;
    println!("{}", {
        let str = 2u128.pow(34).to_string();
        let mut buf = String::with_capacity((str.len() as f32 * 1.34) as usize);
        for (i, ch) in str.chars().enumerate() {
            let i = str.len() - 1 - i;
            buf.push(ch);
            if i % 3 == 0 && i != 0 {
                buf.push('_');
            }
        }
        buf
    });
}
