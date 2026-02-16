// Note: for a full module example, see src/main.rs + src/math.rs
// This file demonstrates inline modules

mod greetings {
    pub fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    pub fn goodbye(name: &str) {
        println!("Goodbye, {}!", name);
    }

    // Private function — can't be called from outside
    fn _secret() {
        println!("this is private");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn mul(a: f32, b: f32) -> f32 {
        a * b
    }
}

// Bring items into scope
use greetings::hello;

fn main() {
    // Full path
    greetings::goodbye("Alice");

    // Short path via `use`
    hello("Bob");

    // Using the math module
    let sum = math::add(2, 3);
    let product = math::mul(2.0, 3.5);
    println!("sum: {}, product: {}", sum, product);
}
