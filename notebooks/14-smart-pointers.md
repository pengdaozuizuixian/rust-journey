# Chapter 14: Smart Pointers

## Beyond References — Heap Allocation & Shared Ownership

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // === BOX<T> — simplest smart pointer ===
    // Stores data on the HEAP instead of the stack
    // Single owner, just like a regular value

    // Why use Box?
    // 1. When you have a large amount of data and want to transfer ownership without copying
    // 2. When you need a type with a known size at compile time (recursive types)
    // 3. When you want to own a value and only care that it implements a trait (trait objects)

    let b = Box::new(5); // 5 is stored on the heap
    println!("b = {}", b); // Box implements Deref, so it works like a regular value

    // Box is most useful for recursive types
    // Without Box, this would have infinite size:
    // enum List { Cons(i32, List), Nil }  // ERROR: infinite size
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    println!("list: {:?}", list);

    // === DEREF TRAIT — treating smart pointers like references ===
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference Box to get the inner value

    // Deref coercion: Rust automatically converts &Box<T> to &T
    let name = Box::new(String::from("Alice"));
    greet(&name); // Box<String> → &String → &str (automatic coercion chain)

    // === DROP TRAIT — cleanup when value goes out of scope ===
    {
        let _c = CustomSmartPointer {
            data: String::from("hello"),
        };
        let _d = CustomSmartPointer {
            data: String::from("world"),
        };
        println!("CustomSmartPointers created");
        // _d is dropped first, then _c (reverse order)
    } // both are dropped here

    // Force early drop with std::mem::drop
    let e = CustomSmartPointer {
        data: String::from("early drop"),
    };
    println!("before drop");
    drop(e); // explicitly drop early
    println!("after drop");
    // println!("{}", e.data); // ERROR: e was dropped

    // === RC<T> — Reference Counted (shared ownership) ===
    // Multiple owners of the same data (single-threaded only!)
    // The data is dropped when the LAST Rc is dropped

    // Scenario: two lists sharing a tail
    //   a = 5 → 10 → Nil
    //   b = 3 → a
    //   c = 4 → a
    // Both b and c own a!

    let a = Rc::new(SharedList::Cons(5,
        Rc::new(SharedList::Cons(10,
            Rc::new(SharedList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let _b = SharedList::Cons(3, Rc::clone(&a)); // clone just increments the count
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2

    let _c = SharedList::Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a)); // 3

    // When _c and _b go out of scope, count decreases
    // When count reaches 0, the data is freed

    // === REFCELL<T> — Interior Mutability ===
    // Allows mutating data even when there are immutable references
    // Borrowing rules are checked at RUNTIME instead of compile time
    // Panics if you violate them!

    let data = RefCell::new(vec![1, 2, 3]);

    // borrow() — immutable borrow (like &T)
    println!("data: {:?}", data.borrow());

    // borrow_mut() — mutable borrow (like &mut T)
    data.borrow_mut().push(4);
    println!("data after push: {:?}", data.borrow());

    // Multiple immutable borrows: OK
    {
        let _r1 = data.borrow();
        let _r2 = data.borrow();
        // let _r3 = data.borrow_mut(); // PANIC at runtime! Can't mix & and &mut
    }

    // === RC<REFCELL<T>> — shared AND mutable ===
    // The most common combo: multiple owners that can all mutate the data

    let shared = Rc::new(RefCell::new(0));

    let owner1 = Rc::clone(&shared);
    let owner2 = Rc::clone(&shared);

    *owner1.borrow_mut() += 10;
    *owner2.borrow_mut() += 20;

    println!("shared value: {}", shared.borrow()); // 30
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// === RECURSIVE TYPE WITH BOX ===
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// === SHARED LIST WITH RC ===
enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

// === CUSTOM DROP ===
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data: {}", self.data);
    }
}
```

## Smart Pointer Cheat Sheet

```
┌─────────────┬──────────────┬────────────┬───────────────────────┐
│ Type        │ Ownership    │ Mutability │ Use case              │
├─────────────┼──────────────┼────────────┼───────────────────────┤
│ Box<T>      │ Single       │ If owned   │ Heap allocation,      │
│             │              │            │ recursive types       │
├─────────────┼──────────────┼────────────┼───────────────────────┤
│ Rc<T>       │ Multiple     │ Read-only  │ Shared data,          │
│             │ (ref count)  │            │ single-threaded       │
├─────────────┼──────────────┼────────────┼───────────────────────┤
│ RefCell<T>  │ Single       │ Interior   │ Mutate behind &T,     │
│             │              │ mutability │ runtime borrow check  │
├─────────────┼──────────────┼────────────┼───────────────────────┤
│ Rc<RefCell> │ Multiple     │ Shared +   │ Multiple owners that  │
│             │              │ mutable    │ can all mutate        │
└─────────────┴──────────────┴────────────┴───────────────────────┘
```

## Key Takeaways

- **`Box<T>`**: puts data on the heap, single owner — needed for recursive types
- **`Rc<T>`**: reference-counted shared ownership (single-threaded only)
- **`RefCell<T>`**: interior mutability — borrow rules checked at runtime, not compile time
- **`Rc<RefCell<T>>`**: multiple owners with mutable access — common pattern
- `Deref` trait lets smart pointers behave like regular references
- `Drop` trait runs cleanup code when a value goes out of scope
- `Rc::clone()` is cheap — just increments a counter, doesn't deep copy
- For multi-threaded shared ownership, use `Arc<T>` instead of `Rc<T>` (next chapter)
