# Chapter 9: Traits

## Shared Behavior (Like Interfaces)

```rust
// A trait defines behavior — a set of methods a type must implement
// Similar to interfaces in Java/TypeScript or protocols in Swift

// === DEFINING A TRAIT ===
trait Greet {
    fn hello(&self) -> String;

    // Traits can have DEFAULT implementations
    fn goodbye(&self) -> String {
        String::from("Goodbye!")
    }
}

// === IMPLEMENTING A TRAIT ===
struct Person {
    name: String,
}

impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hi, I'm {}!", self.name)
    }
    // goodbye() uses the default implementation
}

struct Robot {
    id: u32,
}

impl Greet for Robot {
    fn hello(&self) -> String {
        format!("BEEP BOOP. Unit {} online.", self.id)
    }

    // Override the default
    fn goodbye(&self) -> String {
        format!("Unit {} shutting down.", self.id)
    }
}

// === TRAITS AS PARAMETERS ===
// Accept any type that implements a trait

// Syntax 1: impl Trait (simple, common)
fn print_greeting(item: &impl Greet) {
    println!("{}", item.hello());
}

// Syntax 2: Trait bound (more flexible)
fn print_both<T: Greet>(a: &T, b: &T) {
    println!("{}", a.hello());
    println!("{}", b.hello());
}

// Syntax 3: where clause (cleaner for multiple bounds)
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Greet + std::fmt::Debug,
    U: Greet,
{
    format!("{} and {}", t.hello(), u.hello())
}

// === RETURNING TRAITS ===
fn create_greeter() -> impl Greet {
    Person {
        name: String::from("Alice"),
    }
}

// === COMMON STANDARD LIBRARY TRAITS ===

// Display — for user-facing output (enables {} in println!)
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Clone and Copy
#[derive(Clone, Debug)]
struct Config {
    width: u32,
    height: u32,
}

// PartialEq — enables == and !=
// PartialOrd — enables <, >, <=, >=
#[derive(Debug, PartialEq, PartialOrd)]
struct Score {
    value: u32,
}

fn main() {
    // Using our trait
    let person = Person { name: String::from("Alice") };
    let robot = Robot { id: 42 };

    println!("{}", person.hello());   // Hi, I'm Alice!
    println!("{}", person.goodbye()); // Goodbye! (default)
    println!("{}", robot.hello());    // BEEP BOOP. Unit 42 online.
    println!("{}", robot.goodbye());  // Unit 42 shutting down. (overridden)

    // Trait as parameter
    print_greeting(&person);
    print_greeting(&robot);

    // Display trait
    let p = Point { x: 3.0, y: 4.0 };
    println!("point: {}", p); // uses our Display implementation

    // Clone
    let c1 = Config { width: 800, height: 600 };
    let c2 = c1.clone(); // deep copy
    println!("c1: {:?}, c2: {:?}", c1, c2);

    // PartialEq
    let s1 = Score { value: 100 };
    let s2 = Score { value: 100 };
    let s3 = Score { value: 50 };
    println!("s1 == s2: {}", s1 == s2); // true
    println!("s1 > s3: {}", s1 > s3);   // true

    // Returning a trait
    let greeter = create_greeter();
    println!("{}", greeter.hello());
}

// === DERIVE MACROS — auto-implement common traits ===
// #[derive(Debug)]      — enables {:?} printing
// #[derive(Clone)]      — enables .clone()
// #[derive(Copy, Clone)] — enables implicit copy (stack types only)
// #[derive(PartialEq)]  — enables == and !=
// #[derive(PartialOrd)] — enables <, >, <=, >=
// #[derive(Hash)]       — enables use as HashMap key
// #[derive(Default)]    — enables Type::default()
```

## Key Takeaways

- **Traits** define shared behavior — like interfaces with optional default methods
- `impl TraitName for Type` to implement a trait for a type
- Use `&impl Trait` or `<T: Trait>` to accept any type implementing a trait
- `where` clauses keep complex trait bounds readable
- `impl Trait` in return position lets you return a type without naming it
- **Common derive traits**: `Debug`, `Clone`, `Copy`, `PartialEq`, `PartialOrd`, `Hash`, `Default`
- `Display` trait enables `{}` formatting — must be implemented manually
- Traits are Rust's way of achieving polymorphism without inheritance
