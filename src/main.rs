use rayon::iter::Either;
use rayon::prelude::*;

fn main() {
    let (first, (left, right)): (Vec<_>, (Vec<_>, Vec<_>)) = (0..8)
        .into_par_iter()
        .map(|x| {
            if x % 2 == 0 {
                (x, Either::Left(x * 4))
            } else {
                (-x, Either::Right(x * 3))
            }
        })
        .collect();
    println!("{first:?}");
    println!("{left:?}");
    println!("{right:?}");
}
