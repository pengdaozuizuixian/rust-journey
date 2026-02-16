# Chapter 6: Enums

## Defining and Using Enums

```rust
// An enum defines a type that can be ONE of several variants
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enums can hold data — each variant can have different types!
#[derive(Debug)]
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },    // named fields (like a struct)
    Write(String),               // a single String
    ChangeColor(u8, u8, u8),     // three u8 values (like a tuple)
}

// Enums can have methods too
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quitting"),
            Message::Move { x, y } => println!("moving to ({}, {})", x, y),
            Message::Write(text) => println!("writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("color: ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // === BASIC ENUM ===
    let dir = Direction::North;
    println!("going: {:?}", dir);

    // Match is the most common way to handle enums
    match dir {
        Direction::North => println!("heading north"),
        Direction::South => println!("heading south"),
        Direction::East => println!("heading east"),
        Direction::West => println!("heading west"),
    }

    // === ENUM WITH DATA ===
    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 128);
    let msg4 = Message::Quit;

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    // === OPTION<T> — Rust's replacement for null ===
    // Defined in the standard library:
    // enum Option<T> {
    //     Some(T),  // there IS a value
    //     None,     // there is NO value
    // }

    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // You MUST handle the None case — Rust won't let you ignore it
    // This prevents null pointer errors!

    // Using match
    match some_number {
        Some(n) => println!("got number: {}", n),
        None => println!("no number"),
    }

    // Using if let (when you only care about one variant)
    if let Some(n) = some_number {
        println!("unwrapped: {}", n);
    }

    // Common Option methods
    let x: Option<i32> = Some(10);
    println!("unwrap_or: {}", x.unwrap_or(0));         // returns 10
    println!("is_some: {}", x.is_some());               // true
    println!("is_none: {}", x.is_none());               // false

    let y: Option<i32> = None;
    println!("unwrap_or: {}", y.unwrap_or(0));          // returns 0 (the default)
    // y.unwrap(); // PANIC! — don't use unwrap() on None in production

    // map — transform the inner value if it exists
    let doubled = x.map(|n| n * 2); // Some(20)
    println!("doubled: {:?}", doubled);

    let nothing = y.map(|n| n * 2); // None (map does nothing on None)
    println!("nothing: {:?}", nothing);

    // === RESULT<T, E> — for operations that can fail ===
    // enum Result<T, E> {
    //     Ok(T),   // success with value T
    //     Err(E),  // failure with error E
    // }

    let good: Result<i32, String> = Ok(42);
    let bad: Result<i32, String> = Err(String::from("something went wrong"));

    match good {
        Ok(val) => println!("success: {}", val),
        Err(e) => println!("error: {}", e),
    }

    match bad {
        Ok(val) => println!("success: {}", val),
        Err(e) => println!("error: {}", e),
    }

    // Parsing a string to a number returns Result
    let parsed: Result<i32, _> = "42".parse();
    let failed: Result<i32, _> = "abc".parse();

    println!("parsed: {:?}", parsed);   // Ok(42)
    println!("failed: {:?}", failed);   // Err(...)

    // === WHILE LET ===
    // Loop while a pattern matches
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped: {}", top); // 3, 2, 1
    }
}
```

## Key Takeaways

- Enums define a type with **multiple variants** — a value is exactly one variant
- Variants can hold **different types of data** (no data, tuples, structs)
- Enums can have **methods** via `impl` blocks, just like structs
- `Option<T>` replaces null — forces you to handle the "no value" case
- `Result<T, E>` represents success (`Ok`) or failure (`Err`)
- Use `match` to handle all variants, `if let` for a single variant
- `while let` loops while a pattern keeps matching
- Common `Option` methods: `unwrap_or()`, `map()`, `is_some()`, `is_none()`
