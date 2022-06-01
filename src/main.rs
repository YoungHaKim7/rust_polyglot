// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
// Tuples can be used as function arguments and as return values
// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html

use ndarray::arr2;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // let too_long_tuple = (
    //     1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32, 11i32, 12i32, 13i32,
    // );
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parenthese
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuples can be destuctured to create bindings

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}, ", a, b, c, d);

    let ndarray_a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let ndarray_b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &ndarray_a + &ndarray_b;

    println!("{}", ndarray_a);
    println!("+");
    println!("{}", ndarray_b);
    println!("=");
    println!("{}", sum);
}
