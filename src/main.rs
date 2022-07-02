// https://riptutorial.com/rust/example/24222/displaying-raw-pointers
use std::ptr;

// Create some data, a raw pointer pointing to it and a null pointer
fn main() {
    let data: u32 = 42;
    let raw_ptr = &data as *const u32;
    let null_ptr = ptr::null() as *const u32;

    // the {:p} mapping shows pointer values as hexadecimal memory addresses
    println!("Data address: {:p}", &data);
    println!("Raw pointer address: {:p}", raw_ptr);
    println!("Null pointer address: {:p}", null_ptr);
}
