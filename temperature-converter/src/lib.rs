
pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    if !check_inout(from_unit, to_unit) {
        Err(String::from("Invalid unit"))
    } else {
        Ok(perform_convert(value, from_unit, to_unit))
    }
}

pub fn check_inout(from_unit: &str, to_unit: &str) -> bool {
    match (from_unit, to_unit) {
        ("C" | "K" | "F", "C" | "K" | "F") =>  true,
        _ => false
    }
}


pub  fn perform_convert(value: f64,from_unit: &str, to_unit: &str) -> f64 {
        match (from_unit, to_unit) {
            ("F", "C") => (value - 32.0) * (5.0/9.0) ,
            ("F", "K") => (value - 32.0) * (5.0/9.0) + 273.15,
            ("C", "F") => value * (9.0 / 5.0) + 32.0,
            ("C", "K") => value + 273.15,
            ("K", "C") => value - 273.15,
            ("K", "F") => (value -273.15) * (9.0 / 5.0) + 32.0,
            _ => value

        }
}
