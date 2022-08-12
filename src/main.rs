fn main() {
    let mut s: String = String::new();
    "hello".clone_into(&mut s);

    println!("{s}");

    let mut v: Vec<i32> = Vec::new();
    [1, 2][..].clone_into(&mut v);
    println!("{v:?}");
}
