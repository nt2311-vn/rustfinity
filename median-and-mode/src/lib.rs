use core::f32;
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
    numbers.sort();

}
