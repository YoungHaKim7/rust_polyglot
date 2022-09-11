fn y_xxx(x: i32) -> i32 {
    x * x * x
}

fn main() {
    let my_vec = ((-10..10)).map(|x|(x, y_xxx(x))).collect::<Vec<_>>();
    println!("{my_vec:?}");

}
