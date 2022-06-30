use rust_fibonacci_trait::Fibonacci;

fn main() {
    let fibonacci: Vec<u128> = Fibonacci::new().take(100).collect();
    println!("{fibonacci:?}");
}
