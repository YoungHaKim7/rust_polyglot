use std::time::{Duration, Instant, SystemTime};

// fn fibonacci_recusive(n: i64) -> i64 {
//     if n < 2 {
//         return n;
//     }
//     return fibonacci_recusive(n - 1) + fibonacci_recusive(n - 2);
// }

fn fibonacci_iterative(n: i64) -> i64 {
    let mut first_number: i64 = 0;
    let mut second_number: i64 = 0;
    let mut current_number: i64 = 1;

    let mut i: i64 = 1;

    while i < n {
        first_number = second_number;

        second_number = current_number;

        current_number = first_number + second_number;

        i = i + 1;
    }
    current_number
}

fn main() {
    // let start_sys_time_recursive = Instant::now();
    // println!("{}", fibonacci_recusive(35));
    // println!(
    //     "Recursive {:?} millisec",
    //     start_sys_time_recursive.elapsed().as_millis()
    // );

    let start_sys_time_iterative = Instant::now();

    println!("{}", fibonacci_iterative(50));
    println!(
        "Iterative {:?} millisec",
        start_sys_time_iterative.elapsed().as_millis()
    );
}
