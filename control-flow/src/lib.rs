pub fn check_number_sign(number: i32) -> String {
    // Return `"positive"` if the number is positive.
    // Return `"negative"` if the number is negative.
    // Return `"zero"` if the number is zero.

    // Step 1:
    // Check if the number is positive.

    if number > 0 {
        return "positive".to_string()
    }

    // Step 2:
    // Check if the number is negative.

    if number < 0 {
        return "negative".to_string()
    }

    // Step 3:
    // Handle the case where it's neither positive nor negative.
    return "zero".to_string()
}
