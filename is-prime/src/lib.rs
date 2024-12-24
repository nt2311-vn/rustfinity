pub fn is_prime(n: u32) -> bool {
    match n {
        1 => false,
        2 => true,
        _ => {
            let half:u32 = n / 2 + 1;
            for divisor in 2..half {
                if n % divisor == 0 {
                    return false;
                }
            }

            return true
        } 
    }
}


