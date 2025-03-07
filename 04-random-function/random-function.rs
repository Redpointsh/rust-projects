use rand::Rng;

fn multiple(number: u32) {
    println!("Your number is {} also the result is: {}",number, number * (number - 1))
}

fn main() {
    let random = rand::rng().random_range(1..=3);

    match random {
        1=> multiple(1),
        2=> multiple(2),
        3=> multiple(3),

        _=> panic!("There is a error!")
    }
}