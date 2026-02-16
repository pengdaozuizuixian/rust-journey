fn main() {
    // === CREATING A STRUCT ===
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    println!("{} is {} years old", user1.username, user1.age);

    // === MUTABLE STRUCT ===
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: false,
    };
    user2.active = true; // entire struct must be mut
    println!("{} active: {}", user2.username, user2.active);

    // === STRUCT UPDATE SYNTAX ===
    let user3 = User {
        username: String::from("charlie"),
        ..user2 // take remaining fields from user2
    };
    println!("{} - {}", user3.username, user3.email);

    // === TUPLE STRUCTS ===
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("red: ({}, {}, {})", red.0, red.1, red.2);
    println!("origin: ({}, {})", origin.0, origin.1);

    // === DEBUG PRINTING ===
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect: {:?}", rect);
    println!("rect pretty: {:#?}", rect);

    // === METHODS ===
    println!("area: {}", rect.area());
    println!("is square: {}", rect.is_square());

    // === ASSOCIATED FUNCTIONS (constructors) ===
    let square = Rectangle::square(20);
    println!("square: {:?}, area: {}", square, square.area());

    // === METHOD WITH ANOTHER STRUCT ===
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect2 can hold rect: {}", rect2.can_hold(&rect));

    // === BUILDER FUNCTION ===
    let user4 = build_user(String::from("diana"), String::from("diana@example.com"));
    println!("{} - active: {}", user4.username, user4.active);
}

// === STRUCT DEFINITION ===
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// Tuple structs — named tuples
struct Color(u8, u8, u8);
struct Point(f64, f64);

// Derive Debug for printing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods go in impl blocks
impl Rectangle {
    // &self = immutable borrow of the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // No &self = associated function (like a static method)
    // Called with :: not .
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Function that builds a struct
fn build_user(username: String, email: String) -> User {
    User {
        username,  // shorthand: same as username: username
        email,
        age: 0,
        active: true,
    }
}
