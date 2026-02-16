# Chapter 21: Trait Objects & Dynamic Dispatch

## Runtime Polymorphism with `dyn Trait`

```rust
// === STATIC vs DYNAMIC DISPATCH ===
//
// Static dispatch (generics):  compiler generates code for each type at compile time
//   fn draw<T: Shape>(s: &T)  →  draw_circle(), draw_square() (separate functions)
//   Pro: fast (inlined), Con: larger binary
//
// Dynamic dispatch (trait objects):  method is looked up at runtime via vtable
//   fn draw(s: &dyn Shape)   →  one function, looks up the right method at runtime
//   Pro: flexible, smaller binary, Con: slight runtime cost (pointer indirection)

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> &str {
        "Rectangle"
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    fn name(&self) -> &str {
        "Triangle"
    }
}

fn main() {
    // === TRAIT OBJECTS WITH &dyn ===
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 4.0, height: 6.0 };

    // &dyn Shape — a reference to ANY type that implements Shape
    print_shape(&circle);
    print_shape(&rect);

    // === HETEROGENEOUS COLLECTIONS ===
    // Vec<Box<dyn Shape>> — a vector that holds different types!
    // This is impossible with generics (Vec<T> needs one concrete T)

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
        Box::new(Triangle { base: 6.0, height: 3.0 }),
    ];

    println!("=== All shapes ===");
    let total_area = total_area(&shapes);
    for shape in &shapes {
        println!("  {}: area = {:.2}", shape.name(), shape.area());
    }
    println!("  total area: {:.2}", total_area);

    // === RETURNING TRAIT OBJECTS ===
    let shape = create_shape("circle");
    println!("\ncreated: {} with area {:.2}", shape.name(), shape.area());

    // === TRAIT OBJECTS IN STRUCTS ===
    let screen = Screen {
        components: vec![
            Box::new(Button { label: String::from("OK") }),
            Box::new(TextBox { text: String::from("Hello") }),
            Box::new(Button { label: String::from("Cancel") }),
        ],
    };
    screen.draw();

    // === STATIC DISPATCH COMPARISON ===
    // This function works but ALL items must be the same type
    print_shape_static(&circle);
    print_shape_static(&rect);
    // Can't put circle AND rect in the same Vec with generics
}

// === DYNAMIC DISPATCH — accepts any Shape ===
fn print_shape(shape: &dyn Shape) {
    println!("{}: area = {:.2}", shape.name(), shape.area());
}

// === STATIC DISPATCH — monomorphized at compile time ===
fn print_shape_static<T: Shape>(shape: &T) {
    println!("{}: area = {:.2}", shape.name(), shape.area());
}

// === WORKING WITH Vec<Box<dyn Trait>> ===
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// === RETURNING TRAIT OBJECTS ===
fn create_shape(kind: &str) -> Box<dyn Shape> {
    match kind {
        "circle" => Box::new(Circle { radius: 10.0 }),
        "rectangle" => Box::new(Rectangle { width: 5.0, height: 3.0 }),
        _ => Box::new(Triangle { base: 4.0, height: 2.0 }),
    }
}

// === GUI EXAMPLE — classic trait object use case ===
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("  [Button: {}]", self.label);
    }
}

struct TextBox {
    text: String,
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("  |TextBox: {}|", self.text);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>, // can hold any Draw-able component
}

impl Screen {
    fn draw(&self) {
        println!("=== Drawing screen ===");
        for component in &self.components {
            component.draw();
        }
    }
}

// === OBJECT SAFETY ===
// Not all traits can be used as trait objects. A trait is "object safe" if:
//   1. It doesn't return Self
//   2. It doesn't have generic type parameters in methods
//
// This trait is NOT object safe:
// trait Clone {
//     fn clone(&self) -> Self;  // returns Self — can't use as dyn Clone
// }
//
// This IS object safe:
// trait Draw {
//     fn draw(&self);  // no Self in return, no generics
// }
```

## Static vs Dynamic Dispatch

```
┌──────────────────┬──────────────────────┬──────────────────────┐
│                  │ Static (generics)    │ Dynamic (dyn Trait)  │
├──────────────────┼──────────────────────┼──────────────────────┤
│ Syntax           │ fn foo<T: Trait>(x:T)│ fn foo(x: &dyn Trait)│
│ Resolved at      │ Compile time         │ Runtime              │
│ Performance      │ Faster (inlined)     │ Slight overhead      │
│ Binary size      │ Larger (copies)      │ Smaller              │
│ Mixed types      │ No (one T at a time) │ Yes (heterogeneous)  │
│ Use when         │ Performance matters  │ Flexibility matters  │
└──────────────────┴──────────────────────┴──────────────────────┘
```

## Key Takeaways

- **`dyn Trait`** enables runtime polymorphism — different types in the same collection
- Use **`Box<dyn Trait>`** to own trait objects, **`&dyn Trait`** to borrow them
- **`Vec<Box<dyn Trait>>`** holds mixed types — the classic use case
- Static dispatch (generics) is faster; dynamic dispatch is more flexible
- Trait objects use a **vtable** for method lookup at runtime
- **Object safety**: traits with `Self` returns or generic methods can't be used as `dyn`
- Common pattern: GUI components, plugin systems, strategy pattern
