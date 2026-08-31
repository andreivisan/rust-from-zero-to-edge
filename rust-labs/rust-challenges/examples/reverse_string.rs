use rust_challenges::algorithms::strings::reverse_string::reverse;

fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    println!("{}", reverse(&input));
}
