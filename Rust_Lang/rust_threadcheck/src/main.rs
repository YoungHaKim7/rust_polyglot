fn main() {
    println!(
        "Hello, world! my thread No. {}",
        std::thread::available_parallelism().unwrap()
    );
}
