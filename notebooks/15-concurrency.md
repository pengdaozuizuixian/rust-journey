# Chapter 15: Concurrency

## Fearless Concurrency — Rust's Superpower

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // === SPAWNING THREADS ===
    // thread::spawn takes a closure and runs it in a new thread

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("  spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap(); // blocks until thread completes
    println!("both threads done\n");

    // === MOVE CLOSURES — transferring ownership to threads ===
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // `move` transfers ownership of `data` into this thread
        println!("data in thread: {:?}", data);
    });
    // println!("{:?}", data); // ERROR: data was moved

    handle.join().unwrap();

    // === MESSAGE PASSING WITH CHANNELS ===
    // mpsc = Multiple Producer, Single Consumer
    // Like a one-way pipe: senders → receiver

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["hi", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Receiving messages (blocks until a message arrives)
    for received in rx {
        println!("got: {}", received);
    }
    println!();

    // === MULTIPLE PRODUCERS ===
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // clone the sender for a second thread

    thread::spawn(move || {
        let msgs = vec!["A1", "A2", "A3"];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        let msgs = vec!["B1", "B2", "B3"];
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    for received in rx {
        println!("multi-producer got: {}", received);
    }
    println!();

    // === SHARED STATE WITH MUTEX ===
    // Mutex = Mutual Exclusion — only one thread can access the data at a time
    // You MUST lock the mutex to access the data

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // lock and get mutable access
        *num = 6;
        // lock is automatically released when `num` goes out of scope
    }
    println!("mutex value: {:?}", m);

    // === ARC<MUTEX<T>> — shared mutable state across threads ===
    // Arc = Atomic Reference Counted (like Rc but thread-safe)
    // Arc<Mutex<T>> = multiple threads can own and mutate the same data

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // each thread gets its own Arc
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("final counter: {}", *counter.lock().unwrap()); // 10

    // === SCOPED THREADS (Rust 1.63+) ===
    // Scoped threads can borrow from the parent scope (no move needed!)
    let mut data = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("scoped thread sees: {:?}", data); // borrows data
        });

        s.spawn(|| {
            println!("another scoped thread sees: {:?}", data);
        });
    }); // all scoped threads are joined here automatically

    // data is still accessible here
    data.push(4);
    println!("data after scoped threads: {:?}", data);
}
```

## Concurrency Cheat Sheet

```
┌──────────────────┬───────────────────────────────────────────┐
│ Tool             │ Use case                                  │
├──────────────────┼───────────────────────────────────────────┤
│ thread::spawn    │ Run code in a new thread                  │
│ handle.join()    │ Wait for a thread to finish               │
│ move closure     │ Transfer ownership into a thread          │
├──────────────────┼───────────────────────────────────────────┤
│ mpsc::channel    │ Send messages between threads (one-way)   │
│ tx.send()        │ Send a value through the channel          │
│ rx.recv()        │ Block and wait for a message              │
│ tx.clone()       │ Create multiple senders                   │
├──────────────────┼───────────────────────────────────────────┤
│ Mutex<T>         │ Protect shared data (one thread at a time)│
│ Arc<T>           │ Thread-safe reference counting            │
│ Arc<Mutex<T>>    │ Shared mutable state across threads       │
├──────────────────┼───────────────────────────────────────────┤
│ thread::scope    │ Spawn threads that can borrow local data  │
└──────────────────┴───────────────────────────────────────────┘
```

## Key Takeaways

- Rust prevents data races at **compile time** — fearless concurrency
- `thread::spawn` creates a new thread, `.join()` waits for it
- Use `move` closures to transfer ownership into threads
- **Channels** (`mpsc`) for message passing — like a pipe between threads
- **`Mutex<T>`** for shared mutable data — must `.lock()` before accessing
- **`Arc<T>`** is the thread-safe version of `Rc<T>` (atomic reference counting)
- **`Arc<Mutex<T>>`** = multiple threads owning and mutating the same data
- **Scoped threads** can borrow from the parent scope without `move`
- Two concurrency models: **message passing** (channels) vs **shared state** (Mutex)
