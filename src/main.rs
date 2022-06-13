use rand::distributions::Uniform;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 1000);

    let vals: Vec<u64> = (0..10).map(|_| rng.sample(&range)).collect();

    println!("{:?}", vals);
}
