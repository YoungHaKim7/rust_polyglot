use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let vals: Vec<u64> = (0..1000).map(|_| rng.gen_range(0..1000)).collect();

    println!("{vals:?}");
}
