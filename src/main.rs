fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    std::thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    a.push(4);
    assert_eq!(x, a.len());
}
