fn main() {
    // === BASIC ENUM ===
    let dir = Direction::North;
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
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // Must handle both cases
    match some_number {
        Some(n) => println!("\ngot number: {}", n),
        None => println!("\nno number"),
    }

    // if let — when you only care about one variant
    if let Some(n) = some_number {
        println!("unwrapped: {}", n);
    }

    // Common Option methods
    println!("unwrap_or: {}", some_number.unwrap_or(0));
    println!("is_some: {}", some_number.is_some());
    println!("unwrap_or (None): {}", no_number.unwrap_or(0));

    // map — transform inner value
    let doubled = some_number.map(|n| n * 2);
    let nothing = no_number.map(|n| n * 2);
    println!("doubled: {:?}", doubled);
    println!("nothing: {:?}", nothing);

    // === RESULT<T, E> — for operations that can fail ===
    let parsed: Result<i32, _> = "42".parse();
    let failed: Result<i32, _> = "abc".parse();
    println!("\nparsed: {:?}", parsed);
    println!("failed: {:?}", failed);

    match parsed {
        Ok(val) => println!("success: {}", val),
        Err(e) => println!("error: {}", e),
    }

    // === WHILE LET ===
    let mut stack = vec![1, 2, 3];
    print!("\npopping: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();
}

// === ENUM DEFINITIONS ===

#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Variants can hold different types of data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
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
