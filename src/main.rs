use std::{io, thread};

fn main() -> io::Result<()> {
    let count = thread::available_parallelism()?.get();
    println!("available_parallelism cpu no. {}", count);
    Ok(())
}

