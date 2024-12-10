pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    if start > end {
        return 0;
    }

    let mut sum :i32 = 0;

    for num in start..=end {
        if num % 2 == 0 {
            sum += num;
        }
    }

    return sum;
}
