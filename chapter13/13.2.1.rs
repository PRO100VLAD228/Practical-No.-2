use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20)); 

    let result = multiply("t", "2");
    assert_eq!(result.is_err(), true); 

    println!("Success!");
}
