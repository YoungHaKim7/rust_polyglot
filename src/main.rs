fn main() {
    let s = "abc".to_string();
    let a = [s; 0];

    dbg!(&a);
    // const A: [String; 0] = [String::new(); 0];

    println!("{:?}", a);
}
