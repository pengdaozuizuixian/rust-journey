# Chapter 20: Unsafe Rust

## Breaking Free from the Compiler's Safety Net

```rust
// === WHAT IS UNSAFE RUST? ===
// Rust's compiler is conservative — it rejects some valid programs.
// `unsafe` lets you do 5 things the compiler can't verify:
//   1. Dereference raw pointers
//   2. Call unsafe functions
//   3. Access/modify mutable static variables
//   4. Implement unsafe traits
//   5. Access fields of unions
//
// IMPORTANT: unsafe doesn't disable the borrow checker or other checks.
// It only unlocks these 5 specific capabilities.

fn main() {
    // === 1. RAW POINTERS ===
    // *const T (immutable) and *mut T (mutable)
    // Unlike references: can be null, can point to invalid memory, no borrow rules

    let mut num = 5;

    // Creating raw pointers is safe
    let r1 = &num as *const i32;     // immutable raw pointer
    let r2 = &mut num as *mut i32;   // mutable raw pointer

    // DEREFERENCING raw pointers requires unsafe
    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);

        // You can have both *const and *mut to the same data
        // (not possible with regular references!)
        *r2 = 10;
        println!("after mutation via r2: r1 = {}", *r1); // 10
    }

    // === 2. CALLING UNSAFE FUNCTIONS ===
    unsafe {
        dangerous();
    }

    // Safe wrapper around unsafe code — common pattern
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("left: {:?}", left);   // [1, 2, 3]
    println!("right: {:?}", right); // [4, 5, 6]

    // === 3. MUTABLE STATIC VARIABLES ===
    // Static variables have a fixed memory address for the entire program
    // Mutable statics are unsafe because of potential data races

    add_to_counter(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // === 4. EXTERN FUNCTIONS (FFI) ===
    // Calling C code from Rust
    unsafe {
        println!("abs(-3) from C: {}", abs(-3));
    }

    // === WHEN TO USE UNSAFE ===
    // - Interfacing with C/C++ code (FFI)
    // - Performance-critical code where bounds checking is too slow
    // - Implementing data structures the compiler can't verify (linked lists, etc.)
    // - Working with hardware or OS-level APIs
    //
    // RULES OF THUMB:
    // - Keep unsafe blocks as SMALL as possible
    // - Wrap unsafe code in safe abstractions
    // - Document WHY the unsafe code is actually safe
    // - Prefer safe Rust whenever possible
}

// === UNSAFE FUNCTION ===
unsafe fn dangerous() {
    println!("this function is unsafe to call");
}

// === SAFE WRAPPER AROUND UNSAFE CODE ===
// This is the idiomatic pattern: unsafe internals, safe interface
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // raw pointer to the slice

    assert!(mid <= len); // safety check!

    unsafe {
        // We know these two slices don't overlap, so this is safe
        // But the compiler can't prove it, so we need unsafe
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// === MUTABLE STATIC VARIABLE ===
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// === FFI — calling C functions ===
extern "C" {
    fn abs(input: i32) -> i32;
}

// Exposing Rust functions to C
#[no_mangle] // don't change the function name during compilation
pub extern "C" fn call_from_c() {
    println!("called from C!");
}

// === UNSAFE TRAITS ===
// A trait is unsafe when at least one of its methods has an invariant
// the compiler can't verify
unsafe trait Foo {
    fn bar(&self);
}

unsafe impl Foo for i32 {
    fn bar(&self) {
        println!("bar for i32: {}", self);
    }
}
```

## Key Takeaways

- `unsafe` unlocks **5 specific capabilities**, it doesn't disable all safety
- **Raw pointers** (`*const T`, `*mut T`) can be null, dangling, or aliased
- Keep unsafe blocks **as small as possible** — wrap in safe abstractions
- Common use: **FFI** (calling C), performance-critical code, low-level data structures
- `extern "C"` for calling C functions, `#[no_mangle]` for exposing Rust to C
- Mutable static variables are unsafe due to potential data races
- Rule: if you can do it in safe Rust, do it in safe Rust
- Most Rust developers rarely write `unsafe` — the standard library handles it
