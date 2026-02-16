fn main() {
    // === IF / ELSE ===
    let number = 7;

    if number > 5 {
        println!("greater than 5");
    } else if number == 5 {
        println!("equal to 5");
    } else {
        println!("less than 5");
    }

    // if is an expression
    let label = if number > 5 { "big" } else { "small" };
    println!("{} is {}", number, label);

    // === LOOPS ===

    // loop with break returning a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result: {}", result);

    // while
    let mut n = 3;
    while n > 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!");

    // for with array
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("value: {}", element);
    }

    // for with range
    for i in 1..=3 {
        println!("i = {}", i);
    }

    // === MATCH ===
    let coin = "quarter";
    let value = match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0,
    };
    println!("{} = {} cents", coin, value);

    // Match with multiple patterns
    let x = 3;
    match x {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=10 => println!("four to ten"),
        _ => println!("something else"),
    }

    // Match with enum
    let direction = Direction::North;
    match direction {
        Direction::North => println!("heading north"),
        Direction::South => println!("heading south"),
        Direction::East => println!("heading east"),
        Direction::West => println!("heading west"),
    }

    // === IF LET ===
    let some_value: Option<i32> = Some(42);
    if let Some(v) = some_value {
        println!("got: {}", v);
    }
}

#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}
