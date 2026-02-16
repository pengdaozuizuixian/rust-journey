# Chapter 2: Control Flow & Pattern Matching

## If / Else

```rust
fn main() {
    let number = 7;

    // Basic if/else
    if number > 5 {
        println!("greater than 5");
    } else if number == 5 {
        println!("equal to 5");
    } else {
        println!("less than 5");
    }

    // if is an EXPRESSION — it returns a value
    let label = if number > 5 { "big" } else { "small" };
    println!("{} is {}", number, label);

    // === LOOPS ===

    // loop — runs forever until `break`
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break; // exit the loop
        }
    }
    println!("loop ended at count={}", count);

    // loop can return a value via break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // returns 20
        }
    };
    println!("loop result: {}", result);

    // while loop
    let mut n = 5;
    while n > 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!");

    // for loop — the most common loop in Rust
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("value: {}", element);
    }

    // for with range
    for i in 0..5 {
        println!("i = {}", i); // 0, 1, 2, 3, 4
    }

    // for with inclusive range
    for i in 1..=3 {
        println!("i = {}", i); // 1, 2, 3
    }

    // reverse range
    for i in (1..4).rev() {
        println!("countdown: {}", i); // 3, 2, 1
    }

    // === MATCH ===
    // Like switch but WAY more powerful — must be exhaustive

    let coin = "quarter";
    let value = match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0, // _ is the catch-all (like default)
    };
    println!("{} = {} cents", coin, value);

    // Match with numbers
    let x = 3;
    match x {
        1 => println!("one"),
        2 | 3 => println!("two or three"),  // multiple patterns with |
        4..=10 => println!("four to ten"),   // range pattern
        _ => println!("something else"),
    }

    // Match with enums (most common use)
    let direction = Direction::North;
    match direction {
        Direction::North => println!("heading north"),
        Direction::South => println!("heading south"),
        Direction::East => println!("heading east"),
        Direction::West => println!("heading west"),
    }

    // Match with tuples
    let point = (0, -2);
    match point {
        (0, 0) => println!("at origin"),
        (x, 0) => println!("on x-axis at {}", x),
        (0, y) => println!("on y-axis at {}", y),
        (x, y) => println!("at ({}, {})", x, y),
    }

    // === IF LET ===
    // Shorthand when you only care about one pattern
    let some_value: Option<i32> = Some(42);

    // Instead of writing a full match:
    // match some_value {
    //     Some(v) => println!("got: {}", v),
    //     None => {},
    // }

    // Use if let:
    if let Some(v) = some_value {
        println!("got: {}", v);
    }

    // if let with else
    let empty: Option<i32> = None;
    if let Some(v) = empty {
        println!("got: {}", v);
    } else {
        println!("got nothing");
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}
```

## Key Takeaways

- `if` is an **expression** — it can return values
- Rust has 3 loops: `loop` (infinite), `while` (conditional), `for` (iterator)
- `for` with ranges (`0..5`, `1..=3`) is the most idiomatic loop
- `match` must be **exhaustive** — every possible value must be handled
- `match` supports patterns: literals, ranges (`4..=10`), multiple (`2 | 3`), destructuring
- `_` is the catch-all/wildcard pattern
- `if let` is a shorthand for matching a single pattern
