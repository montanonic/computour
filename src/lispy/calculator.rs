pub fn simplest_calculator(code: &str) -> i32 {
    let chars: Vec<char> = code.chars().collect();
    // Use base 10 for our familiar decimal numbering system.
    let num1 = chars[0].to_digit(10).unwrap() as i32;
    let op = chars[1];
    let num2 = chars[2].to_digit(10).unwrap() as i32;

    // "interpreter"
    match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        fail => panic!("bad operation: {}", fail),
    }
}

pub fn double_all(data: &[i32]) -> Vec<i32> {
    let mut output = Vec::with_capacity(data.len());
    for x in data {
        output.push(x * 2);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simplest_calculator_works() {
        let code = "9+3";
        assert_eq!(simplest_calculator(code), 12);
        let code = "5-9";
        assert_eq!(simplest_calculator(code), -4);
        let code = "7*7";
        assert_eq!(simplest_calculator(code), 49);
        let code = "5/3";
        assert_eq!(simplest_calculator(code), 1);
    }
}
