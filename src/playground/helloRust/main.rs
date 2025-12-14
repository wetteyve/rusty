use std::env;

fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "world".to_string());
    println!("Hello, {}!", input);
}
