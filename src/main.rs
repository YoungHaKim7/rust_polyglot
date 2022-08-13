use std::collections::VecDeque;
use std::io::{Read, Write};

fn main() {
    let mut buf: VecDeque<u8> = VecDeque::new();
    buf.write_all(b"Hello ");
    write!(buf, ", {}", "world");

    let mut s = String::new();
    buf.read_to_string(&mut s);

    println!("{}", &s);

    println!("{}", s.is_empty());
    println!("{}", buf.is_empty());
}
