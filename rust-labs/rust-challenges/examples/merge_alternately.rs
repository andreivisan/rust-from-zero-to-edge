use rust_challenges::algorithms::strings::merge_alternately::merge_alternately;

fn main() {
    let result = merge_alternately("Hello".to_string(), ", Andrei aka Cybermaster!".to_string());
    println!("{result}");
}
