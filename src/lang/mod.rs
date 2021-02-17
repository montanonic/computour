mod lispy_interp;
mod stack_lang;
mod stack_machine;
mod vm;

pub fn main() {
    lispy_interp::run();
}
