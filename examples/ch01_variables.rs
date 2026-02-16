#[allow(unused_variables)]
fn main() {
    // === VARIABLES & MUTABILITY ===
    let x = 5;
    // x = 6; // ERROR: immutable by default

    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y = {}", y);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("max points: {}", MAX_POINTS);

    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z = {}", z);

    // Shadowing can change type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // === SCALAR TYPES ===
    let a: i32 = -42;
    let b: u64 = 100;
    let pi: f64 = 3.14159;
    let is_active: bool = true;
    let letter = 'A';
    let emoji = '🦀';
    println!("a={}, b={}, pi={}, active={}, letter={}, emoji={}", a, b, pi, is_active, letter, emoji);

    // === COMPOUND TYPES ===

    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (tx, ty, tz) = tup;
    println!("tx={}, ty={}, tz={}", tx, ty, tz);
    println!("first element: {}", tup.0);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("first={}, second={}", arr[0], arr[1]);

    let zeros = [0; 5];
    println!("zeros length: {}", zeros.len());

    let months: [&str; 3] = ["Jan", "Feb", "Mar"];
    println!("months: {:?}", months);
}
