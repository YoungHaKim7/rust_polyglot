use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..3 {
            println!("Hello from {} thread!", i);
        }
    });

    for i in 1..3 {
        println!("hello from my main {} thread ", i);
    }
}
