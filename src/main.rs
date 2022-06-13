use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let time_1 = Instant::now();
    let time_2 = Instant::now();
    let async_vec_1: Vec<_> = (0..100).into_par_iter().collect();
    let async_vec_2: Vec<_> = (0..100).into_par_iter().collect();

    for i in async_vec_1.iter() {
        println!("1 Thread Element : {:?}", i);
    }
    let time1_ela = time_1.elapsed();

    for i in async_vec_2.iter() {
        println!("2 Thread Element : {:?}", i);
    }
    let time2_ela = time_2.elapsed();

    println!("{:?}", time1_ela);
    println!("{:?}", time2_ela);
}
