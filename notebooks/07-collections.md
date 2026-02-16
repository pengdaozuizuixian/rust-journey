# Chapter 7: Collections (Vec, HashMap, String)

## Vec — Growable Arrays

```rust
use std::collections::HashMap;

fn main() {
    // === VEC<T> — a growable, heap-allocated array ===

    // Creating vectors
    let mut v1: Vec<i32> = Vec::new(); // empty, with type annotation
    let v2 = vec![1, 2, 3, 4, 5];     // vec! macro with initial values

    // Adding elements (must be mut)
    v1.push(10);
    v1.push(20);
    v1.push(30);
    println!("v1: {:?}", v1); // [10, 20, 30]

    // Accessing elements
    let third = &v2[2];          // index access — panics if out of bounds!
    println!("third: {}", third);

    let maybe = v2.get(2);       // safe access — returns Option<&T>
    match maybe {
        Some(val) => println!("got: {}", val),
        None => println!("index out of bounds"),
    }

    let out_of_bounds = v2.get(100); // returns None instead of panicking
    println!("out of bounds: {:?}", out_of_bounds);

    // Iterating
    for val in &v2 {
        println!("  {}", val);
    }

    // Iterating with mutation
    let mut v3 = vec![1, 2, 3];
    for val in &mut v3 {
        *val *= 10; // dereference to modify
    }
    println!("v3 after mutation: {:?}", v3); // [10, 20, 30]

    // Useful methods
    println!("length: {}", v2.len());
    println!("is empty: {}", v1.is_empty());
    println!("contains 3: {}", v2.contains(&3));

    let mut v4 = vec![3, 1, 4, 1, 5];
    v4.sort();
    println!("sorted: {:?}", v4); // [1, 1, 3, 4, 5]
    v4.dedup(); // remove consecutive duplicates
    println!("deduped: {:?}", v4); // [1, 3, 4, 5]

    // Removing elements
    let mut v5 = vec![10, 20, 30, 40];
    let removed = v5.remove(1); // removes at index 1, shifts rest
    println!("removed: {}, v5: {:?}", removed, v5); // 20, [10, 30, 40]
    v5.pop(); // removes last element
    println!("after pop: {:?}", v5); // [10, 30]

    // === STRING — a growable, UTF-8 encoded text ===

    // Creating strings
    let mut s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string(); // from &str to String

    // Appending
    s1.push_str("hello"); // append a string slice
    s1.push(' ');          // append a single character
    s1.push_str("world");
    println!("s1: {}", s1);

    // Concatenation
    let hello = String::from("hello");
    let world = String::from(" world");
    let combined = hello + &world; // hello is MOVED, world is borrowed
    // println!("{}", hello); // ERROR: hello was moved
    println!("combined: {}", combined);

    // format! macro — doesn't move anything
    let first = String::from("tic");
    let second = String::from("tac");
    let third_s = String::from("toe");
    let formatted = format!("{}-{}-{}", first, second, third_s);
    println!("{}", formatted); // first, second, third_s are all still valid

    // String slicing (be careful with UTF-8!)
    let greeting = String::from("hello");
    let slice = &greeting[0..3]; // "hel"
    println!("slice: {}", slice);

    // Iterating over characters
    for c in "hello".chars() {
        print!("{} ", c);
    }
    println!();

    // String length vs character count
    let emoji_str = String::from("🦀🔥");
    println!("bytes: {}", emoji_str.len());        // 8 (4 bytes per emoji)
    println!("chars: {}", emoji_str.chars().count()); // 2

    // === HASHMAP<K, V> — key-value pairs ===

    // Creating a HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 92);

    // Accessing
    let alice_score = scores.get("Alice"); // returns Option<&V>
    match alice_score {
        Some(score) => println!("Alice: {}", score),
        None => println!("Alice not found"),
    }

    // Iterating
    for (name, score) in &scores {
        println!("  {} => {}", name, score);
    }

    // Overwriting a value
    scores.insert(String::from("Alice"), 150); // replaces 100 with 150

    // Insert only if key doesn't exist
    scores.entry(String::from("Alice")).or_insert(200);   // Alice exists, no change
    scores.entry(String::from("Diana")).or_insert(88);    // Diana doesn't exist, inserts 88
    println!("scores: {:?}", scores);

    // Update based on old value (e.g., word counting)
    let text = "hello world hello rust hello";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // dereference to modify
    }
    println!("word count: {:?}", word_count);

    // Useful methods
    println!("length: {}", scores.len());
    println!("contains Bob: {}", scores.contains_key("Bob"));
    scores.remove("Bob");
    println!("after remove: {:?}", scores);
}
```

## Key Takeaways

- **`Vec<T>`**: growable array — use `push`, `pop`, `get`, iterate with `for x in &v`
- **`String`**: growable UTF-8 text — use `push_str`, `format!`, beware of move on `+`
- **`HashMap<K, V>`**: key-value store — use `insert`, `get` (returns `Option`), `entry().or_insert()`
- All three are **heap-allocated** and grow dynamically
- Use `.get()` for safe access (returns `Option`) instead of `[]` which panics
- `entry().or_insert()` is the idiomatic way to insert-if-absent or update
- Strings are UTF-8: `len()` returns bytes, `.chars().count()` returns characters
