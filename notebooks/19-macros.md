# Chapter 19: Macros

## Code That Writes Code

```rust
// === WHAT ARE MACROS? ===
// Macros generate code at compile time. You've already used many:
//   println!()  vec![]  format!()  assert_eq!()  #[derive(Debug)]
// The ! means "this is a macro, not a function"

fn main() {
    // === DECLARATIVE MACROS (macro_rules!) ===
    // Pattern matching on code — the most common type

    // Our first macro
    say_hello!();

    // Macro with arguments
    println!("{}", double!(5)); // 10

    // === VEC-LIKE MACRO ===
    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec: {:?}", v);

    // === MACRO WITH MULTIPLE PATTERNS ===
    calculate!(5 + 3);
    calculate!(10 - 4);
    calculate!(6 * 7);

    // === HASHMAP MACRO ===
    let scores = map! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 92
    };
    println!("scores: {:?}", scores);

    // === COMMON DERIVE MACROS (attribute macros) ===
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();              // Clone
    println!("{:?}", p1);             // Debug
    println!("{}", p1 == p2);         // PartialEq
    println!("{}", Point::default()); // Default — shows (0, 0)

    // === USEFUL BUILT-IN MACROS ===

    // todo! — marks unfinished code (compiles but panics at runtime)
    // let result = unfinished_function();

    // unimplemented! — similar to todo!, for intentionally unimplemented code
    // let result = not_yet();

    // unreachable! — marks code that should never be reached
    let x = 5;
    let _desc = match x {
        1..=10 => "in range",
        _ => unreachable!("x should always be 1-10"),
    };

    // dbg! — debug print with file, line, and expression
    let a = 2;
    let b = dbg!(a * 3); // prints: [src/main.rs:XX] a * 3 = 6
    let _ = dbg!(b + 1); // prints: [src/main.rs:XX] b + 1 = 7

    // concat! — concatenate literals at compile time
    let s = concat!("hello", " ", "world");
    println!("{}", s);

    // include_str! — include file contents as &str at compile time
    // let contents = include_str!("../README.md");

    // env! — read environment variable at compile time
    let version = env!("CARGO_PKG_VERSION");
    println!("version: {}", version);

    // cfg! — check compile-time configuration
    if cfg!(target_os = "macos") {
        println!("running on macOS");
    }

    // file! and line! — current file and line number
    println!("this is {}:{}", file!(), line!());
}

// === DECLARATIVE MACRO DEFINITIONS ===

// Simple macro — no arguments
macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

// Macro with an expression argument
macro_rules! double {
    ($x:expr) => {
        $x * 2
    };
}

// Macro with repetition (like vec![])
macro_rules! my_vec {
    // Match: zero or more expressions separated by commas
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($x); )* // repeat push for each $x
            v
        }
    };
}

// Macro with multiple match arms
macro_rules! calculate {
    ($a:expr + $b:expr) => {
        println!("{} + {} = {}", $a, $b, $a + $b);
    };
    ($a:expr - $b:expr) => {
        println!("{} - {} = {}", $a, $b, $a - $b);
    };
    ($a:expr * $b:expr) => {
        println!("{} * {} = {}", $a, $b, $a * $b);
    };
}

// HashMap literal macro
macro_rules! map {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}

// Using derive macros
#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// todo! and unimplemented! examples
fn unfinished_function() -> i32 {
    todo!("implement this later") // compiles but panics if called
}

fn not_yet() -> i32 {
    unimplemented!() // same idea, different name
}
```

## Macro Fragment Types

```
┌──────────────┬──────────────────────────────────────┐
│ Fragment     │ Matches                              │
├──────────────┼──────────────────────────────────────┤
│ $x:expr      │ Any expression (5, a + b, foo())     │
│ $x:ident     │ An identifier (variable/fn name)     │
│ $x:ty        │ A type (i32, String, Vec<T>)         │
│ $x:pat       │ A pattern (Some(x), (a, b))          │
│ $x:stmt      │ A statement (let x = 5;)             │
│ $x:block     │ A block ({ ... })                    │
│ $x:literal   │ A literal (42, "hello", true)        │
│ $x:tt        │ A single token tree (anything)       │
└──────────────┴──────────────────────────────────────┘

Repetition: $( ... ),*   — zero or more, comma separated
            $( ... ),+   — one or more, comma separated
            $( ... )?    — zero or one (optional)
```

## Key Takeaways

- Macros generate code at **compile time** — identified by `!` suffix
- **`macro_rules!`** defines declarative macros using pattern matching on code
- Use `$x:expr` to capture expressions, `$x:ident` for identifiers, etc.
- **Repetition** `$( ... ),*` handles variable numbers of arguments
- Built-in macros: `println!`, `vec!`, `dbg!`, `todo!`, `unreachable!`, `cfg!`
- **Derive macros** (`#[derive(...)]`) auto-implement traits like Debug, Clone
- `dbg!()` is great for quick debugging — prints expression + value + location
- Macros are hygienic — variables inside macros don't leak out
