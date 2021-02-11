mod early_vm;

pub fn main() {
    println!("{:08b}", 0b0000_0111);
    println!("{:08b}", (0b0000_0111 >> 0) & 0b0000_0001);
}
