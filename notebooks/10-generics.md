# Chapter 10: Generics

## Writing Code That Works with Any Type

```rust
use std::fmt::Display;

fn main() {
    // === GENERIC FUNCTIONS ===
    // <T> means "any type T"

    // Without generics, you'd need separate functions:
    // fn largest_i32(list: &[i32]) -> &i32 { ... }
    // fn largest_f64(list: &[f64]) -> &f64 { ... }

    // With generics — one function for all comparable types:
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));

    // === GENERIC STRUCTS ===
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.5 };
    println!("int point: ({}, {})", integer_point.x, integer_point.y);
    println!("float point: ({}, {})", float_point.x, float_point.y);

    // Methods on generic structs
    println!("x = {}", integer_point.x());
    println!("distance from origin = {}", float_point.distance_from_origin());

    // Multiple type parameters
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("mixed: ({}, {})", mixed.x, mixed.y);

    // Mixup example — combining different generic types
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3: ({}, {})", p3.x, p3.y); // (5, 'c')

    // === GENERIC ENUMS ===
    // You already know these!
    let some: Option<i32> = Some(42);  // Option<T>
    let ok: Result<i32, String> = Ok(100); // Result<T, E>
    println!("some: {:?}, ok: {:?}", some, ok);

    // Custom generic enum
    let wrapper: Wrapper<String> = Wrapper::Value(String::from("hello"));
    match wrapper {
        Wrapper::Value(v) => println!("wrapped: {}", v),
        Wrapper::Empty => println!("nothing"),
    }

    // === TRAIT BOUNDS — constraining generics ===
    print_it(42);
    print_it("hello");
    print_it(3.14);

    let pair = Pair::new(10, 20);
    pair.cmp_display(); // works because i32 implements Display + PartialOrd
}

// Generic function with trait bound
// T must implement PartialOrd (so we can compare) and Copy (so we can copy values)
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// === GENERIC STRUCT ===
struct Point<T> {
    x: T,
    y: T, // both must be the same type
}

// Methods on a generic struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Methods only for specific types
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Struct with multiple type parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    // Method with different generic types than the struct
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Generic enum
enum Wrapper<T> {
    Value(T),
    Empty,
}

// === TRAIT BOUNDS ===

// Only accept types that implement Display
fn print_it<T: Display>(item: T) {
    println!("item: {}", item);
}

// Multiple trait bounds with where clause
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This impl block only exists for T that is Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the larger is x = {}", self.x);
        } else {
            println!("the larger is y = {}", self.y);
        }
    }
}

// Generics have ZERO runtime cost!
// Rust uses "monomorphization" — at compile time, it generates
// concrete versions for each type you actually use.
// generic_fn::<i32>() and generic_fn::<f64>() become two separate functions.
```

## Key Takeaways

- **Generics** let you write code that works with any type: `fn foo<T>(x: T)`
- Use **trait bounds** (`T: Display + Clone`) to constrain what types are allowed
- `where` clauses are cleaner for complex bounds
- Structs, enums, and methods can all be generic
- You can implement methods **only for specific types** (e.g., `impl Point<f64>`)
- `Option<T>` and `Result<T, E>` are generic enums you already use
- Generics have **zero runtime cost** — Rust generates specialized code at compile time (monomorphization)
