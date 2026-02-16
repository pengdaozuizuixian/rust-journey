# Chapter 3: Ownership & Borrowing

## Ownership

```rust
fn main() {
    // === OWNERSHIP ===
    // Rule 1: Each value has exactly one owner
    // Rule 2: When the owner goes out of scope, the value is dropped (freed)

    // --- Move semantics (heap data like String) ---
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2 — s1 is no longer valid!
    // println!("{}", s1); // ERROR: value borrowed after move
    println!("s2 = {}", s2);

    // Copy types (stack data like i32) are different — they copy, not move
    let a = 5;
    let b = a; // a is copied, both are valid
    println!("a = {}, b = {}", a, b);

    // --- Clone (explicit deep copy) ---
    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy — both are valid
    println!("s3 = {}, s4 = {}", s3, s4);

    // --- Ownership and functions ---
    let s5 = String::from("ownership");
    takes_ownership(s5); // s5 is moved into the function
    // println!("{}", s5); // ERROR: s5 was moved

    let x = 42;
    makes_copy(x); // i32 is Copy, so x is still valid
    println!("x is still valid: {}", x);

    // A function can give ownership back
    let s6 = gives_ownership();
    println!("got ownership: {}", s6);

    // === BORROWING (references) ===
    // Instead of moving, we can BORROW with &

    // --- Immutable references (&T) ---
    let s7 = String::from("borrow me");
    let len = calculate_length(&s7); // borrow s7, don't take ownership
    println!("'{}' has length {}", s7, len); // s7 is still valid!

    // You can have multiple immutable references
    let r1 = &s7;
    let r2 = &s7;
    println!("r1={}, r2={}", r1, r2);

    // --- Mutable references (&mut T) ---
    let mut s8 = String::from("hello");
    change(&mut s8); // pass a mutable reference
    println!("after change: {}", s8);

    // Rule: only ONE mutable reference at a time (prevents data races)
    let m1 = &mut s8;
    // let m2 = &mut s8; // ERROR: cannot borrow as mutable more than once
    println!("m1 = {}", m1);

    // Rule: can't mix mutable and immutable references
    // (immutable refs expect the value won't change under them)

    // === STRING SLICES ===
    let sentence = String::from("hello world");
    let word1 = &sentence[0..5];   // "hello" — a slice borrowing part of sentence
    let word2 = &sentence[6..];    // "world"
    println!("'{}' '{}'", word1, word2);

    // &str is a string slice — it's a reference, not an owner
    let greeting: &str = "hi there"; // string literals are &str
    println!("{}", greeting);
}

fn takes_ownership(s: String) {
    println!("took ownership of: {}", s);
} // s is dropped here — memory freed

fn makes_copy(n: i32) {
    println!("copied value: {}", n);
}

fn gives_ownership() -> String {
    String::from("here you go") // ownership moves to caller
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop anything — it's just a reference

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

## Key Takeaways

- Every value in Rust has exactly **one owner**
- When the owner goes out of scope, the value is **dropped** (memory freed)
- Assigning heap data (like `String`) **moves** ownership — the original is invalidated
- Stack data (like `i32`) implements `Copy` — it's duplicated, not moved
- Use `.clone()` for explicit deep copies of heap data
- **Borrowing** (`&T`) lets you reference a value without taking ownership
- **Mutable borrowing** (`&mut T`) lets you modify a borrowed value
- Rules: only **one** `&mut` at a time, and can't mix `&` and `&mut`
- **String slices** (`&str`) are references to parts of a `String`
