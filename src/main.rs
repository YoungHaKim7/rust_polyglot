use rayon::prelude::*;
use std::f64::consts::*;

pub fn factorial(n: usize) -> f64 {
    let out = (1..=n).into_par_iter().reduce(|| 1, |a, b| a * b);
    out as f64
}

pub fn estimate_pi(iterations: usize) -> f64 {
    let factor = (SQRT_2 * 2.0) / 9801.0;

    let sum = (0..iterations)
        .into_par_iter()
        .map(|i| {
            let k = i as f64;

            let numerator = factorial(4 * i) * (1103.0 + (26390.0 * k));
            let denominator = factorial(i).powf(4.0) * (396_f64).powf(4.0 * k);

            factor * numerator / denominator
        })
        .reduce(|| 0.0, |a, b| a + b);

    1.0 / sum
}

fn main() {
    println!("pi_a: {:.70}", estimate_pi(4));
    println!("pi_c: {:.70}", PI);
}

