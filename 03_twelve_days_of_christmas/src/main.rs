fn days(number: u8) -> String {
    let day = match number {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        _ => "invalid day",
    };
    day.to_string()
}

fn gifts(number: u8) -> String {
    let gift = match number {
        0 => "a partridge in a pear tree",
        1 => "two turtle doves",
        2 => "three french hens",
        3 => "four calling birds",
        4 => "five golden rings",
        _ => "invalid gift",
    };
    gift.to_string()
}

fn main() {
    for count in 0..5 {
        println!("On the {} day of Christmas", days(count));
        println!("my true love gave to me");
        for gift in (0..=count).rev() {
            if gift == 0 && count > 0 {
                print!("..and ");
            }
            println!("{}", gifts(gift));
        }
        println!("");
    }
}
