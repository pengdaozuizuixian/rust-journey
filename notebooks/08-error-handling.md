# Chapter 8: Error Handling

## Rust's Approach: No Exceptions, Use Result and Option

```rust
use std::fs;
use std::io;
use std::io::Read;
use std::num::ParseIntError;

fn main() {
    // === PANIC — unrecoverable errors ===
    // Use for bugs, not expected failures

    // panic!("crash and burn"); // immediately stops the program
    // Also triggers on: array out of bounds, unwrap() on None, etc.

    // === RESULT<T, E> — recoverable errors ===
    // Most functions that can fail return Result

    // Example: reading a file
    let content = fs::read_to_string("hello.txt");
    match content {
        Ok(text) => println!("file content: {}", text),
        Err(e) => println!("couldn't read file: {}", e),
    }

    // === HANDLING ERRORS ===

    // 1. match — most explicit
    let number: Result<i32, _> = "42".parse();
    match number {
        Ok(n) => println!("parsed: {}", n),
        Err(e) => println!("parse error: {}", e),
    }

    // 2. unwrap() — panics on Err (use only in prototyping)
    let n: i32 = "42".parse().unwrap();
    println!("unwrapped: {}", n);
    // "abc".parse::<i32>().unwrap(); // PANIC!

    // 3. expect() — like unwrap but with a custom error message
    let n: i32 = "42".parse().expect("failed to parse number");
    println!("expected: {}", n);

    // 4. unwrap_or() — provide a default
    let n: i32 = "abc".parse().unwrap_or(0);
    println!("unwrap_or: {}", n); // 0

    // 5. unwrap_or_else() — compute default with a closure
    let n: i32 = "abc".parse().unwrap_or_else(|_| {
        println!("parse failed, using default");
        -1
    });
    println!("unwrap_or_else: {}", n); // -1

    // === THE ? OPERATOR — propagating errors ===
    // The star of Rust error handling!

    match read_username_from_file() {
        Ok(name) => println!("username: {}", name),
        Err(e) => println!("error: {}", e),
    }

    match read_username_short() {
        Ok(name) => println!("username (short): {}", name),
        Err(e) => println!("error: {}", e),
    }

    // === COMBINING RESULTS ===
    match multiply_strings("6", "7") {
        Ok(result) => println!("6 * 7 = {}", result),
        Err(e) => println!("error: {}", e),
    }

    match multiply_strings("six", "7") {
        Ok(result) => println!("result: {}", result),
        Err(e) => println!("error: {}", e),
    }

    // === MAP AND AND_THEN ===
    let doubled: Result<i32, _> = "21".parse::<i32>().map(|n| n * 2);
    println!("doubled: {:?}", doubled); // Ok(42)

    let chained: Result<i32, _> = "10"
        .parse::<i32>()
        .and_then(|n| if n > 0 { Ok(n * 2) } else { Err("must be positive".into()) });
    println!("chained: {:?}", chained); // Ok(20)
}

// === PROPAGATING ERRORS WITH ? ===
// ? means: if Ok, unwrap it; if Err, return the Err from this function

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = fs::File::open("username.txt")?; // returns Err if file doesn't exist
    let mut username = String::new();
    file.read_to_string(&mut username)?;             // returns Err if can't read
    Ok(username)
}

// Even shorter — chaining with ?
fn read_username_short() -> Result<String, io::Error> {
    fs::read_to_string("username.txt") // one-liner that does the same thing
}

// ? works with any function that returns Result
fn multiply_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?; // returns Err if parse fails
    let y: i32 = b.parse()?;
    Ok(x * y)
}

// === CUSTOM ERROR TYPES ===
#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(ParseIntError),
    IoError(io::Error),
}

// Implement Display for nice error messages
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "'{}' not found", name),
            AppError::ParseError(e) => write!(f, "parse error: {}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

// Convert from other error types into AppError (enables ? operator)
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::IoError(e)
    }
}

// Now you can use ? with functions that return different error types
fn do_something() -> Result<i32, AppError> {
    let content = fs::read_to_string("data.txt")?; // io::Error -> AppError
    let number: i32 = content.trim().parse()?;      // ParseIntError -> AppError
    Ok(number * 2)
}
```

## Key Takeaways

- Rust has **no exceptions** — use `Result<T, E>` for recoverable errors, `panic!` for bugs
- `?` operator is the idiomatic way to propagate errors — it returns `Err` early
- `?` can only be used in functions that return `Result` (or `Option`)
- `unwrap()` / `expect()` panic on error — fine for prototyping, avoid in production
- `unwrap_or()` and `unwrap_or_else()` provide safe defaults
- `map()` transforms the success value, `and_then()` chains fallible operations
- Custom error enums + `From` implementations enable `?` across different error types
- The `From` trait lets you convert between error types automatically
