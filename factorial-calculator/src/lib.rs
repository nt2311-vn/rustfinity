pub fn factorial(n: u32) -> u128 {
    match n {
        0 => 1 as u128,
        1 => 1 as u128,
        _ => factorial(n-1) * n as u128
    }
}
