#[derive(Debug)]
pub enum Thing {
    A,
    B,
}

impl Drop for Thing {
    fn drop(&mut self) {}
}

fn main() {
    let a = Thing::A;

    println!("{a:?}");
    // let n = Thing::B as i32;
}
