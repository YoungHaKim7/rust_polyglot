use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let time_1 = Instant::now();
    let async_vec: Vec<_> = (0..1_000_000).into_par_iter().collect();
    let time_1_ela = time_1.elapsed();

    let time_2 = Instant::now();
    let sync_vec: Vec<_> = (0..1_000_000).into_iter().collect();
    let time_2_ela = time_2.elapsed();

    // for i in async_vec.iter() {
    //     println!("Element : {:?}", i);
    // }

    println!("async_vec time__rayon : {:?}", time_1_ela);
    println!("sync_vec time : {:?}", time_2_ela);
}
