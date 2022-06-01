use std::ops::Deref;
use std::ops::DerefMut;

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

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

struct Whatever(i32);

impl Drop for Whatever {
    fn drop(&mut self) {
        println!("Dropping Whatever with data: {}!", self.0);
    }
}
fn main() {
    // let x = 5;
    // let mut y = MyBox::new(x);
    // *y = 10;

    // assert_eq!(10, *y);
    let a = Whatever(3);
    std::mem::drop(a);
    let b = Whatever(2);
}
