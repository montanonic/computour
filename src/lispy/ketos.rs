use ketos::{FromValueRef, Interpreter};

pub fn main() {
    // Create an interpreter.
    let interp = Interpreter::new();

    let src = std::fs::read_to_string("src/lispy/main.ket").unwrap();

    // Define a function.
    interp.run_code(&src, None).unwrap();

    // Call the function.
    let result = interp.call("main", vec![]).unwrap();

    // Get a Rust value back.
    // let n = i32::from_value_ref(&result).unwrap();

    println!("{:?}", result);
}
