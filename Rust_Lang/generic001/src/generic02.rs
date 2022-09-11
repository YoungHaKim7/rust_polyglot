fn y_xxx<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x * x
}

fn main() {
    let my_vec = (-10..10).map(|x| (x, y_xxx(x))).collect::<Vec<_>>();
    println!("{my_vec:?}");
}