fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("What is your name?");
    let name = read_line();
    println!("Hello, {}!", name);
}
