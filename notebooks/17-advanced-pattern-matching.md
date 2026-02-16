# Chapter 17: Advanced Pattern Matching

## Beyond Basic Match

```rust
fn main() {
    // === DESTRUCTURING STRUCTS ===
    let point = Point { x: 10, y: 20 };

    let Point { x, y } = point; // destructure into variables
    println!("x={}, y={}", x, y);

    // Rename variables during destructure
    let Point { x: px, y: py } = point;
    println!("px={}, py={}", px, py);

    // Match on struct fields
    match point {
        Point { x: 0, y: 0 } => println!("at origin"),
        Point { x, y: 0 } => println!("on x-axis at {}", x),
        Point { x: 0, y } => println!("on y-axis at {}", y),
        Point { x, y } => println!("at ({}, {})", x, y),
    }

    // === DESTRUCTURING ENUMS WITH DATA ===
    let msg = Message::Move { x: 3, y: 7 };

    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to ({}, {})", x, y),
        Message::Write(text) => println!("write: {}", text),
        Message::Color(r, g, b) => println!("color: ({}, {}, {})", r, g, b),
    }

    // === NESTED DESTRUCTURING ===
    let ((feet, inches), Point { x, y }) = ((6, 1), Point { x: 3, y: -10 });
    println!("height: {}'{}\" at ({}, {})", feet, inches, x, y);

    // === MATCH GUARDS — extra conditions ===
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("{} is less than 5", x),
        Some(x) => println!("{} is 5 or more", x),
        None => println!("nothing"),
    }

    // Guard with OR patterns
    let x = 4;
    match x {
        1 | 2 | 3 if true => println!("one, two, or three"), // guard applies to all
        _ => println!("other"),
    }

    // === @ BINDINGS — bind a value while testing it ===
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_val @ 3..=7 } => {
            // id_val is bound AND we know it's in range 3..=7
            println!("found id in range: {}", id_val);
        }
        Message2::Hello { id: 10..=12 } => {
            // we know id is 10-12 but can't use the value (no binding)
            println!("id in 10-12 range");
        }
        Message2::Hello { id } => {
            println!("other id: {}", id);
        }
    }

    // === IGNORING VALUES ===

    // _ ignores a single value
    let (first, _, third) = (1, 2, 3);
    println!("first={}, third={}", first, third);

    // .. ignores remaining parts
    let point3d = (1, 2, 3, 4, 5);
    let (first, .., last) = point3d;
    println!("first={}, last={}", first, last);

    // Ignore fields in a struct
    let origin = Point3D { x: 0, y: 0, z: 0 };
    let Point3D { x, .. } = origin; // only care about x
    println!("x={}", x);

    // _name — prefixing with _ suppresses "unused variable" warnings
    // but STILL binds the value (unlike plain _)
    let _unused = 42; // no warning

    // === MULTIPLE PATTERNS WITH | ===
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // === RANGES IN PATTERNS ===
    let x = 'c';
    match x {
        'a'..='j' => println!("early letter"),
        'k'..='z' => println!("late letter"),
        _ => println!("something else"),
    }

    let grade = 85;
    match grade {
        90..=100 => println!("A"),
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        0..=59 => println!("F"),
        _ => println!("invalid"),
    }

    // === LET-ELSE (Rust 1.65+) — pattern match or diverge ===
    let config_value = Some("42");

    // Instead of:
    // let val = match config_value {
    //     Some(v) => v,
    //     None => return,
    // };

    // Use let-else:
    let Some(val) = config_value else {
        println!("no config value, exiting");
        return;
    };
    println!("config: {}", val);

    // Great for early returns
    let input = "123";
    let Ok(number) = input.parse::<i32>() else {
        println!("not a number");
        return;
    };
    println!("parsed: {}", number);

    // === MATCHES! MACRO ===
    // Returns true/false if a value matches a pattern
    let foo = 'f';
    println!("is lowercase: {}", matches!(foo, 'a'..='z'));

    let bar = Some(42);
    println!("is some: {}", matches!(bar, Some(_)));

    let status = Status::Error;
    println!("is error: {}", matches!(status, Status::Error));
}

struct Point { x: i32, y: i32 }
struct Point3D { x: i32, y: i32, z: i32 }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

enum Message2 {
    Hello { id: i32 },
}

enum Status { Ok, Error }
```

## Key Takeaways

- **Destructure** structs, enums, tuples, and nested types in patterns
- **Match guards** (`if condition`) add extra conditions to match arms
- **`@` bindings** let you bind a value AND test it: `id @ 3..=7`
- **`_`** ignores a value, **`..`** ignores remaining fields/elements
- **`|`** for multiple patterns, **`..=`** for inclusive ranges
- **`let-else`** for pattern matching with early return on failure
- **`matches!`** macro returns `bool` — great for simple pattern checks in conditions
