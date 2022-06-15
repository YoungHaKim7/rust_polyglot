use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let time_1 = Instant::now();
    let sync_vec_1: Vec<_> = (0..100_000_000).into_iter().collect();
    let time1_ela = time_1.elapsed();
    let time_2 = Instant::now();
    let async_vec_2: Vec<_> = (0..100_000_000).into_par_iter().collect();
    let time2_ela = time_2.elapsed();

    // for i in sync_vec_1.iter() {
    //     println!("sync Thread Element : {i:?}");
    // }

    // for i in async_vec_2.iter() {
    //     println!("async Thread Element : {i:?}");
    // }

    println!("sync_vec : into_iter() = {:?}", time1_ela);
    println!("async_vec : into_par_iter() = {:?}", time2_ela);
}
