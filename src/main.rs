fn main() {
    println!("-----");
    println!("-len()----");
    let stuff = vec![1, 2, 3];
    for i in 0..stuff.len() {
        println!("{}", stuff[i]);
    }

    println!("-----");
    println!("-iter()----");

    for elem in stuff.iter() {
        println!("{}", elem);
    }

    println!("-----");
    println!("-iter_mut()----");

    let mut stuff_clone = stuff.clone();
    for elem in stuff_clone.iter_mut() {
        *elem += 2;
        println!("{}", elem);
    }
}
