use std::io;

struct Options {
    choice: String,
    sign: char,
    conv: char
}

fn build_options(cho: String, sig: char, con: char) -> Options {
    let options = Options {
        choice: cho,
        sign: sig,
        conv: con
    };

    options
}

fn convert_money(options: &Options, amount: f32) {
    const EURO_EXCHANGE: f32 = 0.7;
    const DOLLAR_EXCHANGE: f32 = 1.4;

    let price: f32;

    match options.choice.as_str() {
        "dollar"=> {
            price = EURO_EXCHANGE * amount;
            println!("You got {}{} converted to: {}{}", amount, options.sign, price, options.conv);
        },
        "euro"=> {
            price = DOLLAR_EXCHANGE * amount;
            println!("You got {}{} converted to: {}{}", amount, options.sign, price, options.conv);
        }
        _=> {
            panic!("Wrong money type error!");
        }
    }
}

fn main() {
    println!("
    (0) Write the numbers before options to begin exchange progress! \n
    (1) Dollar to Euro \n
    (2) Euro to Dollar \n");

    let mut in_text = String::new();
    io::stdin().read_line(&mut in_text).expect("There is an error on input! (Currency)");
    let num = in_text.trim().parse().expect("That's not even a number!");

    println!("Write amount!");
    let mut amount_text = String::new();
    io::stdin().read_line(&mut amount_text).expect("There is an error on input! (Amount)");
    let amount = amount_text.trim().parse().expect("This is not a number!");

    let options = match num {
        1 => build_options("dollar".to_string(), '$', '£'),
        2 => build_options("euro".to_string(), '£', '$'),
        _=> {
            println!("You picked wrong number, please try again!");
            return;
        }
    };

    convert_money(&options, amount);
}