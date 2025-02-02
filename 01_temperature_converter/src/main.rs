use std::io::{self, Read};

fn main() {
    let mut buffer = [0, 1];

    loop {
        println!("[C] Convert C° -> F°");
        println!("[F] Convert F° -> C°");

        io::stdin().read_exact(&mut buffer).expect("Try again");

        let user_choice: char = buffer[0].to_ascii_lowercase() as char;

        match user_choice {
            'c' => convert_from_c_to_f(),
            'f' => convert_from_f_to_c(),
            'q' => break,
            _ => println!("[C] or [F], [Q] to quit"),
        }
    }
}

fn convert_from_c_to_f() {
    let mut input = String::new();

    println!("Enter °C");

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line.");

    let degree: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let celsius: f64 = degree;
    let converted: f64 = (celsius * 1.8) + 32.0;

    println!("\n{} °C converted is {} °F\n", celsius, converted);
    return;
}

fn convert_from_f_to_c() {
    let mut input = String::new();

    println!("Enter °F");

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line.");

    let degree: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let fahrenheit: f64 = degree;
    let converted: f64 = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("\n{} °C converted is {} °F\n", fahrenheit, converted);
    return;
}
