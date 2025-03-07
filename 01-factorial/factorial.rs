use std::io;

fn get_factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }

    num * get_factorial(num - 1)
}

fn main() {
    let mut in_value = String::new();
    io::stdin().read_line(&mut in_value).expect("Error while reading input!");

    let num = in_value.trim().parse().expect("That's not even a number");

    println!("{}", get_factorial(num));
}