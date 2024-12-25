use std::collections::HashMap;


pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();

    let len =  numbers.len();
    if len % 2 == 0 {
        let first_mid = len/2;
        let second_mid = len/2 -1;

        (numbers[first_mid] + numbers[second_mid]) as f32/ 2.0
    } else {
        let mid = len/2;
        numbers[mid] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for &number in numbers {
        _ = *map.entry(number).and_modify(|val| *val += 1).or_insert(1);
    }

    let max_frequency = map.values().copied().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, count)| count == max_frequency)
        .map(|(key, _)| key)
        .collect()
}
