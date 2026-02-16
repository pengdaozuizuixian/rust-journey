# Chapter 18: Cargo & Crates

## Rust's Build System and Package Manager

```bash
# === PROJECT MANAGEMENT ===

# Create a new binary project
cargo new my_project
# Creates:
#   my_project/
#   ├── Cargo.toml
#   └── src/
#       └── main.rs

# Create a library project
cargo new my_lib --lib
# Creates:
#   my_lib/
#   ├── Cargo.toml
#   └── src/
#       └── lib.rs

# === BUILD & RUN ===
cargo build              # compile (debug mode)
cargo build --release    # compile (optimized, for production)
cargo run                # build + run
cargo run --release      # build + run (optimized)
cargo check              # check if code compiles (faster than build)

# === TESTING ===
cargo test               # run all tests
cargo test test_name     # run specific test
cargo test -- --show-output  # show println! output

# === OTHER COMMANDS ===
cargo fmt                # format code (install: rustup component add rustfmt)
cargo clippy             # lint code (install: rustup component add clippy)
cargo doc --open         # generate and open documentation
cargo update             # update dependencies to latest compatible versions
cargo clean              # remove build artifacts (target/ directory)
```

## Cargo.toml — Project Configuration

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"          # Rust edition (2015, 2018, 2021, 2024)
authors = ["Your Name <you@example.com>"]
description = "A short description"

# === DEPENDENCIES ===
[dependencies]
serde = "1.0"                           # from crates.io (version ^1.0)
serde_json = "1.0"                      # another crate
tokio = { version = "1", features = ["full"] }  # with features
rand = "0.8"                            # random numbers
clap = { version = "4", features = ["derive"] } # CLI argument parsing

# Git dependency
# my_lib = { git = "https://github.com/user/repo" }

# Local path dependency
# my_lib = { path = "../my_lib" }

# === DEV DEPENDENCIES (only for tests/benchmarks) ===
[dev-dependencies]
# Used only in tests, not compiled into the final binary
# criterion = "0.5"  # benchmarking

# === BUILD PROFILES ===
[profile.dev]
opt-level = 0     # no optimization (fast compile, slow runtime)

[profile.release]
opt-level = 3     # max optimization (slow compile, fast runtime)
```

## Using External Crates

```rust
// After adding to Cargo.toml, just `use` them:

// Random numbers (rand crate)
use rand::Rng;

// Serialization (serde crate)
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    // === USING RAND ===
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(1..=100);
    println!("random number: {}", n);

    let coin: bool = rng.gen();
    println!("coin flip: {}", if coin { "heads" } else { "tails" });

    // === USING SERDE (JSON) ===
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    // Serialize to JSON string
    let json = serde_json::to_string(&user).unwrap();
    println!("json: {}", json);

    // Deserialize from JSON string
    let parsed: User = serde_json::from_str(&json).unwrap();
    println!("parsed: {:?}", parsed);
}
```

## Workspaces — Multi-Package Projects

```
my_workspace/
├── Cargo.toml           # workspace root
├── app/                 # binary crate
│   ├── Cargo.toml
│   └── src/main.rs
├── core_lib/            # library crate
│   ├── Cargo.toml
│   └── src/lib.rs
└── utils/               # another library
    ├── Cargo.toml
    └── src/lib.rs
```

```toml
# Workspace root Cargo.toml
[workspace]
members = [
    "app",
    "core_lib",
    "utils",
]
```

## Popular Crates to Know

```
┌────────────────┬──────────────────────────────────────┐
│ Crate          │ Purpose                              │
├────────────────┼──────────────────────────────────────┤
│ serde          │ Serialization (JSON, YAML, etc.)     │
│ tokio          │ Async runtime                        │
│ reqwest        │ HTTP client                          │
│ axum / actix   │ Web frameworks                       │
│ clap           │ CLI argument parsing                 │
│ anyhow         │ Easy error handling                  │
│ thiserror      │ Custom error types                   │
│ tracing        │ Logging and diagnostics              │
│ rand           │ Random number generation             │
│ chrono         │ Date and time                        │
│ regex          │ Regular expressions                  │
│ sqlx           │ Async SQL database access            │
└────────────────┴──────────────────────────────────────┘
```

## Key Takeaways

- **Cargo** is Rust's build system, package manager, and test runner — all in one
- **`Cargo.toml`** defines your project metadata, dependencies, and build settings
- **crates.io** is the Rust package registry — browse at https://crates.io
- `cargo build` compiles, `cargo run` builds and runs, `cargo check` is fastest
- `--release` flag enables optimizations for production builds
- **Workspaces** organize multi-crate projects with shared dependencies
- `cargo fmt` and `cargo clippy` keep your code clean and idiomatic
- **Semantic versioning**: `"1.0"` means `^1.0` (any compatible 1.x version)
