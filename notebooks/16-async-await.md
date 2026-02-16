# Chapter 16: Async / Await

## Non-blocking I/O for High-Performance Applications

```rust
// Async Rust requires a runtime. The most popular is `tokio`.
// Add to Cargo.toml:
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }

// === WHAT IS ASYNC? ===
// Threads:  OS-level, heavy (~8KB stack each), limited count
// Async:    User-level, lightweight, can have millions of tasks
//
// Use threads for: CPU-heavy work (calculations, compression)
// Use async for:   I/O-heavy work (web servers, file/network I/O, APIs)

// === BASIC ASYNC FUNCTION ===
// `async fn` returns a Future — it does NOTHING until awaited
async fn hello() -> String {
    String::from("hello async world")
}

// Futures are lazy — this function returns immediately without running
async fn fetch_data() -> String {
    // Simulate network delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    String::from("data from server")
}

// === MAIN FUNCTION ===
// Use #[tokio::main] to make main async
#[tokio::main]
async fn main() {
    // .await pauses THIS task until the future completes
    // (but other tasks can run while we wait)
    let greeting = hello().await;
    println!("{}", greeting);

    let data = fetch_data().await;
    println!("{}", data);

    // === SEQUENTIAL vs CONCURRENT ===

    // Sequential — one after another (slow)
    let start = tokio::time::Instant::now();
    let a = slow_operation("A").await;
    let b = slow_operation("B").await;
    println!("sequential: {} + {} in {:?}", a, b, start.elapsed());

    // Concurrent — both at the same time with tokio::join!
    let start = tokio::time::Instant::now();
    let (a, b) = tokio::join!(
        slow_operation("A"),
        slow_operation("B"),
    );
    println!("concurrent: {} + {} in {:?}", a, b, start.elapsed());
    // ~2x faster because both run at the same time!

    // === SPAWNING TASKS ===
    // tokio::spawn runs a future as an independent task (like a lightweight thread)
    let handle = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        42
    });

    // Do other work while the task runs...
    println!("doing other work...");

    // Get the result
    let result = handle.await.unwrap();
    println!("spawned task result: {}", result);

    // === MULTIPLE SPAWNED TASKS ===
    let mut handles = vec![];
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            i * 10
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        println!("task result: {}", result);
    }

    // === SELECT — race multiple futures, take the first ===
    tokio::select! {
        val = slow_operation("fast") => {
            println!("fast finished first: {}", val);
        }
        val = very_slow_operation() => {
            println!("slow finished first: {}", val);
        }
    }
    // Only the winner runs to completion; the other is cancelled

    // === ASYNC WITH ERROR HANDLING ===
    match fetch_url("https://example.com").await {
        Ok(body) => println!("got {} bytes", body.len()),
        Err(e) => println!("error: {}", e),
    }

    // === CHANNELS IN ASYNC ===
    let (tx, mut rx) = tokio::sync::mpsc::channel(32); // buffered channel

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("async channel got: {}", msg);
    }
}

async fn slow_operation(name: &str) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    format!("result_{}", name)
}

async fn very_slow_operation() -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    String::from("slow result")
}

async fn fetch_url(url: &str) -> Result<String, String> {
    // In real code, use reqwest:
    //   let body = reqwest::get(url).await?.text().await?;
    // For this example, simulate it:
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    Ok(format!("response from {}", url))
}
```

## Async Cheat Sheet

```
┌────────────────────┬──────────────────────────────────────────┐
│ Concept            │ Description                              │
├────────────────────┼──────────────────────────────────────────┤
│ async fn           │ Returns a Future (lazy, does nothing     │
│                    │ until awaited)                           │
│ .await             │ Pause and wait for a future to complete  │
│ tokio::spawn       │ Run a future as an independent task      │
│ tokio::join!       │ Run multiple futures concurrently        │
│ tokio::select!     │ Race futures, take the first to complete │
├────────────────────┼──────────────────────────────────────────┤
│ #[tokio::main]     │ Make main() async                        │
│ tokio::time::sleep │ Async sleep (doesn't block the thread)   │
│ tokio::sync::mpsc  │ Async channels for message passing       │
└────────────────────┴──────────────────────────────────────────┘
```

## Key Takeaways

- `async fn` returns a **Future** — it's lazy and does nothing until `.await`ed
- `.await` suspends the current task, letting other tasks run (non-blocking)
- Use **`tokio::join!`** to run futures concurrently (parallel-ish)
- Use **`tokio::spawn`** for independent background tasks
- Use **`tokio::select!`** to race futures and take the first result
- Async is for **I/O-bound** work; threads are for **CPU-bound** work
- Tokio is the most popular async runtime — add it to `Cargo.toml`
- Async channels (`tokio::sync::mpsc`) work like `std::sync::mpsc` but non-blocking
- Futures are zero-cost abstractions — compiled into state machines
