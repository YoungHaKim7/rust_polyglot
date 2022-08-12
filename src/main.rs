fn main() {
    let array: [usize; 7] = std::array::from_fn(|i| 100 * i);
    println!("{array:?}");
}
