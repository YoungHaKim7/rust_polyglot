use std::collections::VecDeque;
use std::io::{Read, Write};

fn main() {
    let mut buf: VecDeque<u8> = VecDeque::new();
    buf.write_all(b"Hello");
    write!(buf, " , {}!", "world");
    let ptr = buf.as_slices().0.as_ptr();
    let my_vec = Vec::from(buf);
    let my_vec_string = std::str::from_utf8(&my_vec).unwrap();

    println!("{my_vec_string:?}");
}
