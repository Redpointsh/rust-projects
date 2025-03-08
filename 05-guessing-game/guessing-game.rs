use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let random = rand::rng().random_range(1..100);

    let mut correct = false;
    let mut guesses = 0;

    while !correct {
        println!("Write your guess:");
        guesses += 1;
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).expect("E");
        let guess = input_text.trim().parse::<i32>().expect("N");

        let difference = (random - guess).abs();

        match difference {
            0=> println!("Correct"),
            1 ..= 10 => println!("Almost"),
            11 ..= 25 => println!("Getting closer!"),
            26 ..= 50 => println!("You're gonna lost yourself!"),
            _=> println!("Wrong")
        }

        match guesses.cmp(&random) {
            Ordering::Less => println!("{} is too low try again!", guess),
            Ordering::Greater => println!("{} is too high try again!", guess),
            Ordering::Equal => {
                println!("{} is the magic number.", random);
                println!("You got the magic number in {} guesses.", guesses);
                correct = true;
            }
        }
    }
}