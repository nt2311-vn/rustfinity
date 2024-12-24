pub fn parse_percentage(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(number) => {
            if  number <= 100 {
                Ok(number)
            } else {
                Err(String::from("Percentage out of range"))
            }
        },
        Err(_) => Err(String::from("Invalid input"))
}
}
