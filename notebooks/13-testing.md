# Chapter 13: Testing

## Writing and Running Tests in Rust

```rust
// Tests live in the same file as your code (or in a tests/ directory)

// === THE CODE TO TEST ===
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// === UNIT TESTS ===
// Convention: put tests in a `tests` module at the bottom of the file
#[cfg(test)]  // only compile this module when running tests
mod tests {
    use super::*; // import everything from the parent module

    // === BASIC TEST ===
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    // === ASSERTION MACROS ===
    #[test]
    fn test_assertions() {
        // assert! — checks that a condition is true
        assert!(add(2, 2) == 4);

        // assert_eq! — checks equality (shows both values on failure)
        assert_eq!(add(2, 3), 5);

        // assert_ne! — checks inequality
        assert_ne!(add(2, 3), 6);

        // Custom failure messages
        let result = greeting("Alice");
        assert!(
            result.contains("Alice"),
            "greeting didn't contain name, got: {}",
            result
        );
    }

    // === TESTING STRUCTS ===
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    // === TESTING RESULT ===
    #[test]
    fn test_divide() {
        let result = divide(10.0, 2.0);
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
    }

    // Tests can also return Result (no need for assert macros)
    #[test]
    fn test_divide_result() -> Result<(), String> {
        let result = divide(10.0, 2.0)?;
        if result == 5.0 {
            Ok(())
        } else {
            Err(format!("expected 5.0, got {}", result))
        }
    }

    // === TESTING PANICS ===
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("this test should panic");
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_specific_panic() {
        let v = vec![1, 2, 3];
        let _ = v[99]; // panics with "index out of bounds"
    }

    // === IGNORING TESTS ===
    #[test]
    #[ignore] // skip this test unless specifically requested
    fn expensive_test() {
        // ... some slow test
        assert_eq!(2 + 2, 4);
    }
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output (println! is captured by default)
cargo test -- --show-output

# Run a specific test by name
cargo test test_add

# Run tests matching a pattern
cargo test divide

# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored

# Run tests in a specific file
cargo test --lib          # only unit tests
cargo test --test integration_test  # specific integration test
```

## Integration Tests

```
my_project/
├── src/
│   └── lib.rs           # library code
├── tests/               # integration tests directory
│   └── integration_test.rs
└── Cargo.toml
```

```rust
// tests/integration_test.rs
// No need for #[cfg(test)] — the tests/ directory is only for tests

use my_project; // import your crate as an external user would

#[test]
fn test_add_from_outside() {
    assert_eq!(my_project::add(2, 3), 5);
}
```

## Test Organization Summary

```
Unit tests:     Same file as code, in #[cfg(test)] mod tests { }
                Test private functions, fast, focused

Integration:    tests/ directory at project root
                Test public API, like an external user

Doc tests:      Code examples in /// doc comments
                Ensures documentation stays accurate
```

## Key Takeaways

- Mark tests with `#[test]`, group them in `#[cfg(test)] mod tests`
- **Assert macros**: `assert!`, `assert_eq!`, `assert_ne!` — all support custom messages
- `#[should_panic]` tests that code panics as expected
- Tests can return `Result<(), E>` to use the `?` operator
- `#[ignore]` skips slow tests — run them with `cargo test -- --ignored`
- `cargo test` compiles and runs all tests, `cargo test name` filters by name
- **Unit tests** go in the same file, **integration tests** go in `tests/` directory
- Use `-- --show-output` to see `println!` output from passing tests
