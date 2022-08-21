use std::io;

const WELCOME: &str = "Hello, there! Please enter 1 if you want to calculate Fh to C. 2 if you want to get nth Fibonnaci number";

fn main() {
    let mut chosen_option = String::new();

    println!("{}", WELCOME);
    io::stdin()
        .read_line(&mut chosen_option)
        .expect("couldnt read line");

    let chosen_option = chosen_option.trim();

    let mut chosen_number = String::new();
    io::stdin().read_line(&mut chosen_number).expect("asd");
    let chosen_number = chosen_number.trim().parse().expect("wow");

    match chosen_option {
        "1" => println!("{}", fahrenheit_to_celcius(chosen_number)),
        "2" => println!("{}", fib_n(chosen_number)),
        _ => (),
    }
}

fn fahrenheit_to_celcius(fh_degrees: i32) -> i32 {
    (fh_degrees - 32) * 5 / 9
}

fn fib_n(n: i32) -> i32 {
    let mut curr = 1;
    let mut last = 0;
    let mut res = 0;
    let mut counter: i32 = 1;

    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => return 1,
        3 => return 2,
        _ => (),
    }

    while counter <= n {
        res = curr + last;
        last = curr;
        curr = res;
        counter += 1;
    }

    return res;
}
