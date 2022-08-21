use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..=13);
    println!("secret number is: {number}");
    println!("Please enter your number");

    loop {
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("failed to read line");

        let guess_number: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_number.cmp(&number) {
            Ordering::Equal => {
                println!("won");
                break;
            }
            _ => println!("lost"),
        };
    }
}
