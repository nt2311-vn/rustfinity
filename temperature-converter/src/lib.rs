
enum TUnit {
    C,
    F,
    K,
    Invalid
}

impl From<&str> for TUnit {
    fn from(value: &str) -> TUnit {
        match value  {
            "C" => TUnit::C,
            "F" => TUnit::F,
            "K" => TUnit::K,
            _ =>  TUnit::Invalid
        }
    }
}

pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let from = TUnit::from(from_unit);
    let to = TUnit::from(to_unit);

    match (from, to) {
        (TUnit::Invalid, _) | (_, TUnit::Invalid) => Err(String::from("Invalid unit")),
        (TUnit::F, TUnit::C) => Ok((value - 32.0) * (5.0/9.0)),
        (TUnit::F, TUnit::K) => Ok((value - 32.0) * (5.0/9.0) + 273.15),
        (TUnit::C, TUnit::F) => Ok(value * (9.0 / 5.0) + 32.0),
        (TUnit::C, TUnit::K) => Ok(value + 273.15),
        (TUnit::K, TUnit::C) => Ok(value - 273.15),
        (TUnit::K, TUnit::F) => Ok((value -273.15) * (9.0 / 5.0) + 32.0),
        _ => Ok(value)
    }
    
}


