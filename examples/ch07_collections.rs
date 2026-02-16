use std::collections::HashMap;

fn main() {
    // === VEC<T> — growable array ===
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v: {:?}", v);

    // vec! macro shorthand
    let v2 = vec![1, 2, 3, 4, 5];

    // Access by index (panics if out of bounds!)
    let third = &v2[2];
    println!("third: {}", third);

    // Safe access with .get() (returns Option)
    let maybe = v2.get(2);
    println!("get(2): {:?}", maybe);     // Some(3)
    println!("get(99): {:?}", v2.get(99)); // None

    // Iterating
    print!("v2: ");
    for val in &v2 {
        print!("{} ", val);
    }
    println!();

    // Mutating while iterating
    let mut v3 = vec![1, 2, 3];
    for val in &mut v3 {
        *val *= 10; // dereference to modify
    }
    println!("v3 mutated: {:?}", v3);

    // Useful methods
    let mut v4 = vec![3, 1, 4, 1, 5];
    v4.sort();
    println!("sorted: {:?}", v4);
    println!("contains 4: {}", v4.contains(&4));
    println!("len: {}", v4.len());

    v4.pop(); // remove last
    println!("after pop: {:?}", v4);

    // === STRING — growable UTF-8 text ===
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("s: {}", s);

    // From &str
    let s2 = String::from("hello");
    let s3 = "world".to_string();

    // Concatenation with format! (doesn't move anything)
    let combined = format!("{} {}", s2, s3);
    println!("combined: {}", combined);
    println!("s2 still valid: {}", s2); // s2 not moved!

    // String length: bytes vs chars
    let emoji = String::from("🦀🔥");
    println!("bytes: {}, chars: {}", emoji.len(), emoji.chars().count());

    // Iterating over characters
    print!("chars: ");
    for c in "hello".chars() {
        print!("{} ", c);
    }
    println!();

    // === HASHMAP<K, V> — key-value pairs ===
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 92);

    // Access (returns Option)
    if let Some(score) = scores.get("Alice") {
        println!("\nAlice: {}", score);
    }

    // Iterating
    println!("all scores:");
    for (name, score) in &scores {
        println!("  {} => {}", name, score);
    }

    // Overwrite
    scores.insert(String::from("Alice"), 150);

    // Insert only if key doesn't exist
    scores.entry(String::from("Alice")).or_insert(200);  // no change
    scores.entry(String::from("Diana")).or_insert(88);   // inserted
    println!("after entry: {:?}", scores);

    // Word counting — classic HashMap pattern
    let text = "hello world hello rust hello";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word count: {:?}", word_count);
}
