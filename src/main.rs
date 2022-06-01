use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 // tuple  tuple.0
    }
    // *y -> *(y.deref())
}
fn hello(name: &str) {
    println!("Hello, {}! ", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust "));
    hello(&(*m)[..]);
    // hello(&m);
}
