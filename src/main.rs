use std::collections::VecDeque;
fn main() {
    let mut a = VecDeque::from([1, 2, 3]);

    a.push_back(6);

    let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();

    println!("{doubled:?}");
}
