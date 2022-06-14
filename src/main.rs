// Rayon
// Test in parallel if any or all elements of a collection match a given predicate
use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];
    println!("vec![2, 4, 6, 8]");
    println!("!vec.par_iter().any(|n| (*n % 2) != 0 ) = {}",!vec.par_iter().any(|n| (*n % 2) != 0 ));
    println!("false 0 , true 1");
    println!("vec.par_iter().all(|n| (*n % 2) == 0 ) = {}",vec.par_iter().any(|n| (*n % 2) == 0 ));
    println!("!vec.par_iter().any(|n| (*n > 8) = {}",!vec.par_iter().any(|n| *n > 8 ));
    println!("vec.par_iter().all(|n| (*n <= 8) = {}",vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);
    println!("----------------\nvec![2, 4, 6, 8, 9]");
    println!("vec.par_iter().any(|n| (*n % 2) != 0 ) = {}",vec.par_iter().any(|n| (*n % 2) != 0 ));
    println!("!vec.par_iter().all(|n| (*n % 2) == 0 ) = {}",!vec.par_iter().all(|n| (*n % 2) == 0 ));
    println!("vec.par_iter().any(|n| (*n > 8) = {}",vec.par_iter().any(|n| *n > 8 ));
    println!("!vec.par_iter().all(|n| (*n <= 8) = {}",!vec.par_iter().all(|n| *n <= 8 ));

}
