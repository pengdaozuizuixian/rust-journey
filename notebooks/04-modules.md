# Chapter 4: Modules

## Organizing Code with Modules

```rust
// === src/main.rs ===
mod math; // declares the module — Rust looks for src/math.rs

fn main() {
    let sum = math::add(2, 3);
    println!("Sum: {}", sum);

    let product = math::mul(2.0, 3.5);
    println!("Product: {}", product);
}
```

```rust
// === src/math.rs ===
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn mul(a: f32, b: f32) -> f32 {
    a * b
}
```

## How Modules Work

```rust
// Declare a module inline
mod greetings {
    pub fn hello() {
        println!("Hello!");
    }

    fn secret() {
        println!("This is private");
    }
    // secret() can't be called from outside this module
}

fn main() {
    greetings::hello();     // OK — it's pub
    // greetings::secret(); // ERROR — it's private
}
```

## File Structure

```
src/
├── main.rs          // crate root — declares modules with `mod`
├── math.rs          // module file — contains pub functions
└── utils/           // nested module as a directory
    ├── mod.rs       // required — declares the utils module
    └── helpers.rs   // submodule
```

```rust
// To use nested modules:
// src/main.rs
mod utils;
use utils::helpers; // bring submodule into scope

fn main() {
    helpers::do_something();
}
```

## The `use` Keyword

```rust
mod math;

// Instead of writing math::add() every time:
use math::add;
// or bring everything in:
// use math::*;

fn main() {
    let sum = add(2, 3); // no need for math:: prefix
    println!("Sum: {}", sum);
}
```

## Key Takeaways

- `mod name;` tells Rust to look for `src/name.rs` or `src/name/mod.rs`
- Functions/structs are **private by default** — use `pub` to make them public
- `use` brings items into scope so you don't need the full path
- `::` is the path separator (like `.` in other languages)
- Modules help organize code and control visibility
