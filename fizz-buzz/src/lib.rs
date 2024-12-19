pub fn fizz_buzz(num: u32) -> String {
    let mut rs = String::new();

    if num % 3 == 0 {
        rs += "Fizz";
    }

    if num % 5 == 0 {
        rs += "Buzz";
    }

    if rs.len() == 0 {
        num.to_string()
    } else {
        rs
    }
}
