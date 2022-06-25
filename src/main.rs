// https://lkarev.medium.com/calculating-30000-pi-digits-in-10-seconds-using-multi-threaded-programming-cc417d00a217

use rug::ops::Pow;
use rug::Float;
use std::ops::Div;
use std::result;

fn main() {
    pub static DEFAULT_PRECISION: u32 = 10_000;

    let mut sum: Float = Float::with_val(DEFAULT_PRECISION, 0.0);
    let max_elements: u32 = 10;

    for n in 0..max_elements {
        let first_fraction_num: Float = Float::with_val(DEFAULT_PRECISION, Float::factorial(4 * n));
        let first_fraction_denom: Float = Float::with_val(
            DEFAULT_PRECISION,
            Float::with_val(DEFAULT_PRECISION, 4).pow(n)
                * Float::with_val(DEFAULT_PRECISION, Float::factorial(n)),
        )
        .pow(4);

        let second_fraction_num: Float = Float::with_val(DEFAULT_PRECISION, 23690 * n + 1103);
        let second_fraction_denom: Float = Float::with_val(
            DEFAULT_PRECISION,
            Float::with_val(DEFAULT_PRECISION, 99).pow(4 * n),
        );
        let first_fraction: Float = first_fraction_num / first_fraction_denom;
        let second_fraction: Float = second_fraction_num / second_fraction_denom;

        let sum_element: FLoat = first_fraction * second_fraction;
        sum += sum_element;
    }

    let first_const: Float = Float::with_val(DEFAULT_PRECISION, 8).sqrt()
        / Float::with_val(DEFAULT_PRECISION, 99).pow(2);
    let result = Float::with_val(DEFAULT_PRECISION, 1) / (first_const * sum);

    println!("Pi is {}", result);
}
