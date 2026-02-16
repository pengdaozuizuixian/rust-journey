use std::num::ParseIntError;

fn main() {
    // === RESULT<T, E> — recoverable errors ===

    // Parsing returns Result
    let good: Result<i32, _> = "42".parse();
    let bad: Result<i32, _> = "abc".parse();
    println!("good: {:?}", good); // Ok(42)
    println!("bad: {:?}", bad);   // Err(...)

    // Handle with match
    match good {
        Ok(n) => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }

    // === WAYS TO HANDLE ERRORS ===

    // unwrap() — panics on Err (prototyping only!)
    let n: i32 = "42".parse().unwrap();
    println!("unwrap: {}", n);

    // expect() — like unwrap but with custom message
    let n: i32 = "42".parse().expect("should be a number");
    println!("expect: {}", n);

    // unwrap_or() — safe default
    let n: i32 = "abc".parse().unwrap_or(0);
    println!("unwrap_or: {}", n);

    // unwrap_or_else() — compute default
    let n: i32 = "abc".parse().unwrap_or_else(|_| {
        println!("  parse failed, using -1");
        -1
    });
    println!("unwrap_or_else: {}", n);

    // === THE ? OPERATOR ===
    // Propagates errors — returns Err early if it fails
    match multiply_strings("6", "7") {
        Ok(result) => println!("\n6 * 7 = {}", result),
        Err(e) => println!("\nerror: {}", e),
    }

    match multiply_strings("six", "7") {
        Ok(result) => println!("six * 7 = {}", result),
        Err(e) => println!("six * 7 error: {}", e),
    }

    // === MAP AND AND_THEN ===
    let doubled: Result<i32, _> = "21".parse::<i32>().map(|n| n * 2);
    println!("\ndoubled: {:?}", doubled); // Ok(42)

    let failed: Result<i32, _> = "abc".parse::<i32>().map(|n| n * 2);
    println!("failed map: {:?}", failed); // Err(...)

    // and_then — chain operations that can each fail
    let chained = "10"
        .parse::<i32>()
        .and_then(|n| if n > 0 { Ok(n * 2) } else { Err(make_error()) });
    println!("chained: {:?}", chained); // Ok(20)

    // === PRACTICAL EXAMPLE ===
    println!("\n=== User Input Validator ===");
    for input in &["25", "abc", "-5", "200"] {
        match validate_age(input) {
            Ok(age) => println!("  '{}' → valid: {}", input, age),
            Err(e) => println!("  '{}' → error: {}", input, e),
        }
    }
}

// ? operator: if Err, return it immediately; if Ok, unwrap it
fn multiply_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?; // returns Err if fails
    let y: i32 = b.parse()?;
    Ok(x * y)
}

fn make_error() -> ParseIntError {
    "".parse::<i32>().unwrap_err()
}

// Practical: validate user input
fn validate_age(input: &str) -> Result<u32, String> {
    let age: i32 = input.parse().map_err(|_| format!("'{}' is not a number", input))?;

    if age < 0 {
        return Err(format!("age can't be negative: {}", age));
    }
    if age > 150 {
        return Err(format!("age too large: {}", age));
    }

    Ok(age as u32)
}
