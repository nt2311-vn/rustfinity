pub fn describe_number(n: i32) -> String {
    if n == 0 {
        return String::from("Zero")
    }

    match n > 0 {
        true =>  {
            if n % 2 == 0 {
                String::from("Positive even")
            } else {
                String::from("Positive odd")
            }
        },
            false =>  {
                if n % 2 == 0 {
                    String::from("Negative even")
                } else {
                    String::from("Negative odd")
                }
            }
    }
    
    
}
