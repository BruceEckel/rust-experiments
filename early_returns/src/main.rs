use std::num::ParseIntError;

fn multiply(first_str: &str, second_str: &str) ->
    Result<i32, ParseIntError> {

    // Nested/local function:
    fn extract(number_str: &str) -> Result<i32, ParseIntError> {
        // Question mark means: "If error, return with that error"
        Ok(number_str.parse::<i32>()?)
    }

    let first_number = match extract(first_str) {
        Ok(first_number) => first_number,
        Err(e) => return Err(e), // Returns from multiply() with error
    };

    let first = extract(first_str)?;  // Same as first_number

    let second_number = second_str.parse::<i32>()?;

    Ok(first_number * second_number + first)
}

fn print(result: Result<i32, ParseIntError>) {
    println!("{:?}", result);
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Woe! {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
