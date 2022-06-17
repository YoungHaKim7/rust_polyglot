// Sort a vector in parallel
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let mut rng = thread_rng();
    let mut v: Vec<u64> = (0..20_000_000)
        .map(|_| rng.gen_range(0..20_000_000))
        .collect();
    let time_1 = Instant::now();
    v.par_sort_unstable();
    let time_1_ela = time_1.elapsed();
    println!("par_sort time : {:?}", time_1_ela);

    let mut v1: Vec<u64> = (0..20_000_000)
        .map(|_| rng.gen_range(0..20_000_000))
        .collect();
    let time_2 = Instant::now();
    v1.sort();
    let time_2_ela = time_2.elapsed();
    println!("sort time : {:?}", time_2_ela);
}
