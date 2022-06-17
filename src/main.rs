use std::sync::atomic::{AtomicUsize, Ordering};
use std::{sync::Arc, thread};
fn main() {
    let data = vec![1, 2, 3, 4];

    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();

    thread::spawn(move || {
        other_idx.fetch_add(10, Ordering::SeqCst);
    });

    println!("{}", data[idx.load(Ordering::SeqCst)]);
}
