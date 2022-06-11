use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from {} thread!", i);
        }
    });

    for i in 1..10 {
        println!("hello from my main {} thread ", i);
    }

    handle.join().unwrap();
}
