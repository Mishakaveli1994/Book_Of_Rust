use core::f32;
use std::io;

fn main() {
    // convert_c_to_fahrenheit();

    // let mut number = String::new();
    // println!("Please provide a fibonacci number");

    // io::stdin()
    //     .read_line(&mut number)
    //     .expect("Failed to read number");

    // let number: u32 = match number.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => return,
    // };

    // println!("{}", fibonacci(number));

    twelve_days_of_christmas();
}

fn convert_c_to_fahrenheit() {
    let mut c = String::new();
    println!("Please provide degrees in C");

    io::stdin().read_line(&mut c).expect("Failed to read line");

    let c: f32 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let fahrenheit = c * 1.8 + 32.0;
    println!("{}C => {}F", c, fahrenheit);
}

fn fibonacci(number: u32) -> u32 {
    if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        let mut result: u32 = 0;
        let mut previous: u32 = 0;
        let mut current: u32 = 1;
        for _ in 1..number {
            result = previous + current;
            previous = current;
            current = result;
        }
        result
    }
}

fn twelve_days_of_christmas() -> () {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas,\nMy true love sent to me",
            days[i]
        );
        for b in (0..i + 1).rev() {
            println!(
                "{}{}{}",
                gifts[b],
                if b == 0 { "" } else { "," },
                if b == 1 { " and" } else { "" },
            )
        }
        println!()
    }
}
