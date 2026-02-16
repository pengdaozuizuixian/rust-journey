# Chapter 5: Structs

## Defining and Using Structs

```rust
// A struct groups related data together (like a class without methods... almost)
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    // === CREATING AN INSTANCE ===
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    println!("{} is {} years old", user1.username, user1.age);

    // === MUTABLE STRUCT ===
    // The entire struct must be mutable — you can't mark individual fields as mut
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: false,
    };
    user2.active = true; // OK because user2 is mut
    println!("{} active: {}", user2.username, user2.active);

    // === STRUCT UPDATE SYNTAX ===
    // Create a new struct reusing fields from another
    let user3 = User {
        username: String::from("charlie"),
        ..user2 // take the rest of the fields from user2
        // NOTE: user2.email is MOVED here (String), so user2.email is no longer valid
        // but user2.age and user2.active are Copy types, so they're still valid
    };
    println!("{} - {}", user3.username, user3.email);

    // === TUPLE STRUCTS ===
    // Named tuples — useful when you want a type name but don't need field names
    struct Color(u8, u8, u8);
    struct Point(f64, f64, f64);

    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0, 0.0);
    println!("red: ({}, {}, {})", red.0, red.1, red.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Even though both have 3 fields, Color and Point are different types!
    // You can't pass a Color where a Point is expected.

    // === UNIT STRUCT ===
    // No fields at all — useful for traits (covered later)
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // === PRINTING STRUCTS WITH DEBUG ===
    // Add #[derive(Debug)] to print structs
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect: {:?}", rect);       // one-line debug format
    println!("rect: {:#?}", rect);      // pretty-printed debug format

    // === METHODS ===
    // Functions attached to a struct, defined in an `impl` block
    println!("area: {}", rect.area());
    println!("is square: {}", rect.is_square());

    // === ASSOCIATED FUNCTIONS (like static methods) ===
    // Don't take &self — called with :: instead of .
    let square = Rectangle::square(20);
    println!("square: {:?}, area: {}", square, square.area());

    // === METHOD CHAINING ===
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
}

#[derive(Debug)] // allows {:?} printing
struct Rectangle {
    width: u32,
    height: u32,
}

// impl block — where you define methods for a struct
impl Rectangle {
    // Method: first parameter is always &self (or &mut self, or self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (no &self) — like a constructor
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// You can have multiple impl blocks for the same struct
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// === FUNCTION THAT BUILDS A STRUCT ===
fn build_user(username: String, email: String) -> User {
    User {
        username,  // field init shorthand — same as username: username
        email,     // same as email: email
        age: 0,
        active: true,
    }
}
```

## Key Takeaways

- Structs group related data with **named fields**
- The entire struct is mutable or immutable — no per-field mutability
- **Struct update syntax** (`..other`) copies/moves fields from another instance
- **Tuple structs** have types but no field names — useful for newtype patterns
- `#[derive(Debug)]` enables `{:?}` printing for structs
- **Methods** are defined in `impl` blocks and take `&self` as the first parameter
- **Associated functions** (no `&self`) are called with `::` — often used as constructors
- `self` = takes ownership, `&self` = immutable borrow, `&mut self` = mutable borrow
