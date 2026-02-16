fn main() {
    // === OWNERSHIP ===
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{}", s1); // ERROR: value borrowed after move
    println!("s2 = {}", s2);

    // Copy types
    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b);

    // Clone
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Ownership and functions
    let s5 = String::from("ownership");
    takes_ownership(s5);
    // println!("{}", s5); // ERROR: s5 was moved

    let x = 42;
    makes_copy(x);
    println!("x is still valid: {}", x);

    let s6 = gives_ownership();
    println!("got ownership: {}", s6);

    // === BORROWING ===
    let s7 = String::from("borrow me");
    let len = calculate_length(&s7);
    println!("'{}' has length {}", s7, len);

    // Multiple immutable references
    let r1 = &s7;
    let r2 = &s7;
    println!("r1={}, r2={}", r1, r2);

    // Mutable references
    let mut s8 = String::from("hello");
    change(&mut s8);
    println!("after change: {}", s8);

    // === STRING SLICES ===
    let sentence = String::from("hello world");
    let word1 = &sentence[0..5];
    let word2 = &sentence[6..];
    println!("'{}' '{}'", word1, word2);

    let greeting: &str = "hi there";
    println!("{}", greeting);
}

fn takes_ownership(s: String) {
    println!("took ownership of: {}", s);
}

fn makes_copy(n: i32) {
    println!("copied value: {}", n);
}

fn gives_ownership() -> String {
    String::from("here you go")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
