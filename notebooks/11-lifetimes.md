# Chapter 11: Lifetimes

## Ensuring References Are Always Valid

```rust
fn main() {
    // === THE PROBLEM LIFETIMES SOLVE ===
    // Rust needs to know how long references are valid to prevent dangling references

    // This would be a dangling reference (Rust prevents it):
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // x is dropped at end of this block
    // }
    // println!("{}", r); // ERROR: x doesn't live long enough

    // === LIFETIME ANNOTATIONS ===
    // Lifetimes tell Rust how the lifetimes of references relate to each other
    // Syntax: 'a (apostrophe + lowercase name)

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("longest: {}", result);
        // result must be used HERE, inside this block,
        // because string2 is dropped at the end of this block
    }

    // === LIFETIME IN CONTEXT ===
    // This works because string1 outlives the scope where result is used
    let string3 = String::from("hello");
    let result2;
    {
        let string4 = String::from("world!!");
        result2 = longest(string3.as_str(), string4.as_str());
        println!("longest: {}", result2);
    }

    // === STRUCTS WITH REFERENCES ===
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let sentences: Vec<&str> = novel.split('.').collect();
        first_sentence = sentences[0];
    }
    // This works because first_sentence borrows from novel, which is still alive
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("excerpt: {}", excerpt.part);
    excerpt.announce_and_return("Notice:");

    // === STATIC LIFETIME ===
    // 'static means the reference lives for the entire program
    let s: &'static str = "I have a static lifetime";
    // String literals are always 'static because they're stored in the binary
    println!("{}", s);
}

// === LIFETIME ANNOTATIONS IN FUNCTIONS ===
// 'a means: the returned reference will live at least as long as
//           the SHORTER of the two input lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// If we only return x, we only need x's lifetime
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// === LIFETIME ANNOTATIONS IN STRUCTS ===
// "An ImportantExcerpt can't outlive the reference it holds in part"
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods on structs with lifetimes
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Lifetime elision: Rust infers that the return borrows from &self
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

// === LIFETIME ELISION RULES ===
// Rust can often INFER lifetimes, so you don't always need to write them.
// Three rules the compiler follows:
//
// Rule 1: Each reference parameter gets its own lifetime
//   fn foo(x: &str, y: &str) → fn foo<'a, 'b>(x: &'a str, y: &'b str)
//
// Rule 2: If there's exactly one input lifetime, it's assigned to all outputs
//   fn foo(x: &str) -> &str → fn foo<'a>(x: &'a str) -> &'a str
//
// Rule 3: If one parameter is &self or &mut self, its lifetime is assigned to outputs
//   fn method(&self, x: &str) -> &str → borrows from self
//
// If the compiler can't figure it out after these rules, you must add annotations.

// Examples where you DON'T need lifetime annotations:
fn first_word(s: &str) -> &str {
    // Rule 2 applies: one input → output gets same lifetime
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

// === COMBINING GENERICS, TRAITS, AND LIFETIMES ===
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Key Takeaways

- **Lifetimes** ensure references don't outlive the data they point to
- Syntax: `'a` — a lifetime parameter (just a label, like generic `T`)
- `fn foo<'a>(x: &'a str, y: &'a str) -> &'a str` means: the return value lives as long as the shorter input
- Structs holding references need lifetime annotations: `struct Foo<'a> { field: &'a str }`
- **Lifetime elision**: Rust infers lifetimes in most cases — you only annotate when it can't
- `'static` means the reference lives for the entire program (string literals are `'static`)
- Lifetimes are a **compile-time check** — zero runtime cost
- When in doubt: if a function returns a reference, it must come from an input reference
