// use std::sync::Arc;

use std::sync::Mutex;

static S: Mutex<String> = Mutex::new(String::new());

fn do_a_call() {
    let h = "hello";
    S.lock().unwrap().push_str(h);
    println!("{S:?}");
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();
}
