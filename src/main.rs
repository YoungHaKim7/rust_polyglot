use std::ops::Mul;

fn y_xxx<T>(x: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    x * x * x
}

fn main() {
    let my_vec = ((-10..=10).map(|x| (x, y_xxx(x)))).collect::<Vec<_>>();

    println!("{my_vec:?}");
}
