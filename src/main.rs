// #[derive(Debug)]
// struct Foo {}

fn floor_divide(num: f32, by: f32) -> Result<i32, String> {
    if by == 0. {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok((num / by).floor() as i32)
    }
}

fn main() {
    // 6) Option (v.s. null & java. util. Optional)
    // let foo: Option<Foo> = Option::None;

    // 7) Result (v.s Exception)
    match floor_divide(10., 4.) {
        Ok(result) => println!("result : {:?} ", result),
        Err(error) => println!("ERROR CAUGHT - {} ", error),
    }
}
