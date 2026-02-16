# Chapter 1: Variables & Data Types

## Variables and Mutability

```rust
fn main() {
    // Variables are immutable by default
    let x = 5;
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // Use `mut` to make a variable mutable
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y = {}", y);

    // === CONSTANTS ===
    // Must have type annotation, always immutable, can be declared in any scope
    const MAX_POINTS: u32 = 100_000;
    println!("max points: {}", MAX_POINTS);

    // === SHADOWING ===
    // Re-declare a variable with the same name (creates a NEW variable)
    let z = 5;
    let z = z + 1;       // z is now 6
    let z = z * 2;       // z is now 12
    println!("z = {}", z);

    // Shadowing can change the type (mut cannot!)
    let spaces = "   ";         // &str
    let spaces = spaces.len();  // now usize
    println!("spaces = {}", spaces);

    // === SCALAR TYPES ===

    // Integers: i8, i16, i32, i64, i128, isize (signed)
    //           u8, u16, u32, u64, u128, usize (unsigned)
    let a: i32 = -42;
    let b: u64 = 100;
    let c: isize = 10; // pointer-sized, depends on architecture
    println!("a={}, b={}, c={}", a, b, c);

    // Floating point: f32, f64 (default is f64)
    let pi: f64 = 3.14159;
    let half: f32 = 0.5;
    println!("pi={}, half={}", pi, half);

    // Boolean
    let is_active: bool = true;
    let is_done = false;
    println!("active={}, done={}", is_active, is_done);

    // Character (4 bytes, supports Unicode)
    let letter = 'A';
    let emoji = '🦀';
    println!("letter={}, emoji={}", letter, emoji);

    // === COMPOUND TYPES ===

    // Tuple: fixed-length, can hold different types
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (tx, ty, tz) = tup; // destructuring
    println!("tx={}, ty={}, tz={}", tx, ty, tz);
    println!("first element: {}", tup.0); // access by index

    // Array: fixed-length, same type, stack-allocated
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("first={}, second={}", first, second);

    // Array with same value repeated
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("zeros length: {}", zeros.len());

    // Type-annotated array
    let months: [&str; 3] = ["Jan", "Feb", "Mar"];
    println!("months: {:?}", months);
}
```

## Key Takeaways

- Variables are **immutable by default** — use `mut` to make them mutable
- **Shadowing** lets you re-declare a variable (even with a different type)
- **Constants** (`const`) must have a type annotation and are always immutable
- Rust has **scalar types**: integers, floats, booleans, characters
- Rust has **compound types**: tuples (mixed types) and arrays (same type, fixed size)
- `i32` is the default integer type, `f64` is the default float type
