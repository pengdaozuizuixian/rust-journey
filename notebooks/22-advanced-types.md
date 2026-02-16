# Chapter 22: Advanced Types

## Newtype, Type Aliases, and More

```rust
use std::fmt;
use std::ops::Add;

fn main() {
    // === NEWTYPE PATTERN ===
    // Wrap an existing type in a single-field tuple struct
    // Gives you a NEW type with no runtime cost

    // Problem: both are just Strings — easy to mix up!
    // fn register(name: String, email: String) { ... }
    // register(email, name); // Oops! Swapped arguments, but compiles!

    // Solution: newtype wrappers
    let name = Username(String::from("alice"));
    let email = Email(String::from("alice@example.com"));
    register(name, email); // Can't accidentally swap them!

    // Newtype for adding traits to external types
    // Can't impl Display for Vec<T> directly (orphan rule)
    // But can wrap it:
    let list = PrintableVec(vec![1, 2, 3]);
    println!("list: {}", list); // uses our Display impl

    // Newtype for units/validation
    let meters = Meters(100.0);
    let more_meters = Meters(50.0);
    let total = meters + more_meters; // can implement ops
    println!("total: {} meters", total.0);

    // === TYPE ALIASES ===
    // Creates an alternative name for an existing type
    // NOT a new type — just shorthand

    type Kilometers = i32;
    type Meters2 = i32;

    let distance: Kilometers = 5;
    let height: Meters2 = 10;
    // These ARE the same type — you can mix them (no safety!)
    let _sum = distance + height; // compiles fine

    // Useful for long types
    type Result2<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    fn do_something() -> Result2<String> {
        Ok(String::from("done"))
    }

    println!("{:?}", do_something());

    // === THE NEVER TYPE (!) ===
    // A type that can never be created — for functions that never return

    // panic! returns ! (never type)
    // loop without break returns !
    // std::process::exit returns !

    // Useful in match arms:
    let input = "42";
    let num: i32 = match input.parse() {
        Ok(n) => n,
        Err(_) => panic!("not a number"), // ! coerces to i32
    };
    println!("num: {}", num);

    // continue also returns !
    let values = vec!["1", "two", "3", "four", "5"];
    let numbers: Vec<i32> = values
        .iter()
        .filter_map(|v| v.parse().ok())
        .collect();
    println!("parsed numbers: {:?}", numbers);

    // === DYNAMICALLY SIZED TYPES (DST) ===
    // Types whose size isn't known at compile time

    // str (not &str) is a DST — we always use it behind a reference
    // [T] (not &[T]) is a DST — a slice without known length
    // dyn Trait is a DST — we use &dyn Trait or Box<dyn Trait>

    // The Sized trait: most types are Sized (known size at compile time)
    // Generics are implicitly Sized:
    //   fn foo<T>(t: T)  actually means  fn foo<T: Sized>(t: T)

    // To opt out (accept DSTs):
    //   fn foo<T: ?Sized>(t: &T)  // T might not be Sized

    // This function accepts both &str and &String
    print_ref("hello");
    print_ref(&String::from("world"));

    // === ENUM VARIANTS AS TYPES (common patterns) ===

    // State machine pattern
    let order = Order::new("Pizza");
    let order = order.pay(); // Pending → Paid
    let order = order.ship(); // Paid → Shipped
    println!("order status: {}", order.status());

    // Builder pattern with types
    let config = ConfigBuilder::new()
        .width(800)
        .height(600)
        .title("My App")
        .build();
    println!("config: {}x{} - {}", config.width, config.height, config.title);
}

// === NEWTYPE DEFINITIONS ===
struct Username(String);
struct Email(String);

fn register(name: Username, email: Email) {
    println!("registered: {} <{}>", name.0, email.0);
}

// Newtype that implements Display for Vec
struct PrintableVec<T: fmt::Display>(Vec<T>);

impl<T: fmt::Display> fmt::Display for PrintableVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let items: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", items.join(", "))
    }
}

// Newtype with operator overloading
#[derive(Debug, Clone, Copy)]
struct Meters(f64);

impl Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

// === ?Sized — accepting DSTs ===
fn print_ref<T: ?Sized + fmt::Display>(t: &T) {
    println!("{}", t);
}

// === TYPE STATE PATTERN ===
// Use the type system to enforce valid state transitions

struct Pending;
struct Paid;
struct Shipped;

struct Order<State> {
    item: String,
    _state: std::marker::PhantomData<State>,
}

impl Order<Pending> {
    fn new(item: &str) -> Order<Pending> {
        Order {
            item: item.to_string(),
            _state: std::marker::PhantomData,
        }
    }

    fn pay(self) -> Order<Paid> {
        println!("  {} paid", self.item);
        Order {
            item: self.item,
            _state: std::marker::PhantomData,
        }
    }
}

impl Order<Paid> {
    fn ship(self) -> Order<Shipped> {
        println!("  {} shipped", self.item);
        Order {
            item: self.item,
            _state: std::marker::PhantomData,
        }
    }
}

impl Order<Shipped> {
    fn status(&self) -> String {
        format!("{} has been delivered!", self.item)
    }
}

// Can't call .ship() on Pending, or .pay() on Shipped — compiler enforces it!
// order.ship(); // ERROR if order is Order<Pending>

// === BUILDER PATTERN ===
struct Config {
    width: u32,
    height: u32,
    title: String,
}

struct ConfigBuilder {
    width: u32,
    height: u32,
    title: String,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            width: 640,
            height: 480,
            title: String::from("Untitled"),
        }
    }

    fn width(mut self, w: u32) -> Self {
        self.width = w;
        self
    }

    fn height(mut self, h: u32) -> Self {
        self.height = h;
        self
    }

    fn title(mut self, t: &str) -> Self {
        self.title = t.to_string();
        self
    }

    fn build(self) -> Config {
        Config {
            width: self.width,
            height: self.height,
            title: self.title,
        }
    }
}
```

## Key Takeaways

- **Newtype pattern**: wrap a type in a tuple struct for type safety with zero cost
- Use newtypes to: prevent argument mix-ups, add traits to external types, enforce units
- **Type aliases** (`type Name = ...`) are just shorthand — not new types
- **Never type** (`!`) for functions that never return (panic, loop, exit)
- **DSTs** (str, [T], dyn Trait) must always be behind a pointer (&, Box, Rc)
- **`?Sized`** opts out of the implicit `Sized` bound to accept DSTs
- **Type state pattern**: use generics to enforce valid state transitions at compile time
- **Builder pattern**: method chaining with `self` return for readable object construction
