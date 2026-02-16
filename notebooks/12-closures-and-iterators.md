# Chapter 12: Closures & Iterators

## Closures — Anonymous Functions That Capture Their Environment

```rust
fn main() {
    // === CLOSURES ===
    // Like anonymous functions / lambdas / arrow functions

    // Basic syntax
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // Rust can infer types, and single expressions don't need braces
    let double = |x| x * 2;
    println!("double(5) = {}", double(5));

    // Closures CAPTURE variables from their environment
    let name = String::from("Alice");
    let greet = || println!("Hello, {}!", name); // captures `name` by reference
    greet();
    println!("name still valid: {}", name); // name is still usable

    // Capture by mutable reference
    let mut count = 0;
    let mut increment = || {
        count += 1; // captures `count` by &mut
        println!("count: {}", count);
    };
    increment(); // 1
    increment(); // 2
    increment(); // 3
    // Can use count again after increment is done
    println!("final count: {}", count);

    // Capture by value with `move`
    let data = vec![1, 2, 3];
    let owns_data = move || {
        println!("data: {:?}", data); // data is MOVED into the closure
    };
    owns_data();
    // println!("{:?}", data); // ERROR: data was moved

    // === CLOSURES AS FUNCTION PARAMETERS ===
    let numbers = vec![1, 2, 3, 4, 5];

    // map — transform each element
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled); // [2, 4, 6, 8, 10]

    // filter — keep elements that match a condition
    let evens: Vec<&i32> = numbers.iter().filter(|x| **x % 2 == 0).collect();
    println!("evens: {:?}", evens); // [2, 4]

    // === ITERATORS ===
    // Iterators are lazy — they don't do anything until consumed

    let v = vec![1, 2, 3];

    // .iter()      — iterates over &T (borrows)
    // .iter_mut()  — iterates over &mut T (mutable borrows)
    // .into_iter() — iterates over T (takes ownership)

    // Creating an iterator (lazy — nothing happens yet)
    let iter = v.iter();

    // Consuming it
    for val in iter {
        print!("{} ", val);
    }
    println!();

    // === ITERATOR ADAPTORS (lazy, chainable) ===
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Chain multiple operations — nothing runs until .collect() or another consumer
    let result: Vec<i32> = v
        .iter()
        .filter(|&&x| x % 2 == 0)   // keep even numbers
        .map(|&x| x * x)             // square them
        .collect();                    // consume and collect into Vec
    println!("even squares: {:?}", result); // [4, 16, 36, 64, 100]

    // === CONSUMING ADAPTORS (trigger execution) ===
    let v = vec![1, 2, 3, 4, 5];

    // sum
    let total: i32 = v.iter().sum();
    println!("sum: {}", total); // 15

    // count
    let count = v.iter().count();
    println!("count: {}", count); // 5

    // any — true if any element matches
    let has_even = v.iter().any(|&x| x % 2 == 0);
    println!("has even: {}", has_even); // true

    // all — true if all elements match
    let all_positive = v.iter().all(|&x| x > 0);
    println!("all positive: {}", all_positive); // true

    // find — returns first match as Option
    let first_even = v.iter().find(|&&x| x % 2 == 0);
    println!("first even: {:?}", first_even); // Some(2)

    // position — returns index of first match
    let pos = v.iter().position(|&x| x == 3);
    println!("position of 3: {:?}", pos); // Some(2)

    // min, max
    println!("min: {:?}", v.iter().min()); // Some(1)
    println!("max: {:?}", v.iter().max()); // Some(5)

    // fold — accumulate a result (like reduce in JS)
    let product = v.iter().fold(1, |acc, &x| acc * x);
    println!("product: {}", product); // 120

    // === CHAINING EXAMPLE — real-world style ===
    let words = vec!["hello", "world", "foo", "bar", "hello", "world"];

    let long_unique: Vec<&&str> = words
        .iter()
        .filter(|w| w.len() > 3)   // keep words longer than 3 chars
        .collect::<Vec<_>>();       // collect
    println!("long words: {:?}", long_unique);

    // === ENUMERATE — get index + value ===
    let fruits = vec!["apple", "banana", "cherry"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", i, fruit);
    }

    // === ZIP — combine two iterators ===
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![100, 85, 92];
    let leaderboard: Vec<(&&str, &i32)> = names.iter().zip(scores.iter()).collect();
    println!("leaderboard: {:?}", leaderboard);

    // === RANGES AS ITERATORS ===
    let sum: i32 = (1..=100).sum(); // sum 1 to 100
    println!("1+2+...+100 = {}", sum); // 5050

    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("squares: {:?}", squares); // [1, 4, 9, 16, 25]
}

// === CLOSURES AS FUNCTION PARAMETERS ===
// Three traits for closures:
// Fn   — borrows captured variables (can be called multiple times)
// FnMut — mutably borrows captured variables
// FnOnce — takes ownership (can only be called once)

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn apply_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn apply_once<F: FnOnce() -> String>(f: F) -> String {
    f() // can only call once
}
```

## Key Takeaways

- **Closures** are anonymous functions: `|args| expression`
- Closures **capture** variables from their environment (by reference, mut reference, or ownership)
- Use `move` to force a closure to take ownership of captured variables
- **Iterators** are lazy — chained operations don't execute until consumed
- `.iter()` borrows, `.into_iter()` takes ownership, `.iter_mut()` borrows mutably
- **Adaptors** (lazy): `map`, `filter`, `enumerate`, `zip`, `take`, `skip`
- **Consumers** (execute): `collect`, `sum`, `count`, `any`, `all`, `find`, `fold`
- Closure traits: `Fn` (borrow), `FnMut` (mutable borrow), `FnOnce` (ownership)
- Iterators are **zero-cost abstractions** — as fast as hand-written loops
