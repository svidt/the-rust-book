use std::env;

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 1;

    if env::args().len() != 2 {
        println!("Please provide a number for the Fibonacci sequence.\n>> Fibonacci (num)");
        return;
    }

    let input = env::args().nth(1).unwrap();

    let fibonacci: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Try again");
            return;
        }
    };

    println!("0");
    println!("1");

    let mut counter: i32 = fibonacci;

    while counter > 0 {
        let z = { x + y };
        // let z = fibonacci(x, y);
        println!("{}", z);
        x = y;
        y = z;
        counter -= 1;
    }
}

fn fibonacci(x: i32, y: i32) -> i32 {
    let z = x + y;
    return z;
}
