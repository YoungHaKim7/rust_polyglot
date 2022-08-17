pub trait Hypot<T> {
    fn hypot(self, b: T) -> f64;
}

impl<T> Hypot<T> for T
where
    T: core::ops::Mul<T, Output = T>
        + core::ops::Add<T, Output = T>
        + core::convert::Into<f64>
        + Copy,
{
    fn hypot(self, b: T) -> f64 {
        (self * self + b * b).into().sqrt()
    }
}

fn main() {
    let x = 3.0_f64;
    let y = 4.0_f64;
    let c = x.hypot(y);

    println!("hypo : {c}");
}

