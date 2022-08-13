use std::collections::VecDeque;
use std::io::{Read, Write};

fn main() {
    let mut buf: VecDeque<u8> = VecDeque::new();
    buf.write_all(b"Hello ");
    write!(buf, ", {}", "world");

    println!("{buf:?}");
}
