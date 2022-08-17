pub fn hypot<T>(a: T, b: T) -> f64
where
    T: core::ops::Mul<T, Output = T>
        + core::ops::Add<T, Output = T>
        + core::convert::Into<f64>
        + Copy,
{
    (a * a + b * b).into().sqrt()
}

fn main() {
    let x = 3.0_f64;
    let y = 4.0_f64;
    let c = x.hypot(y);

    println!("hypot : {c}");
}

