fn tree(num: u32) -> String {
    let mut buf = String::with_capacity(num as usize);

    for _ in 0..num {
        buf.push('*');
    }
    buf
}

fn main() {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("F");
    let number = input_text.trim().parse().expect("C");

    for i in 0..= number {
        println!("{}", tree(number - i))
    }
}