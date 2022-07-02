use std::arch::asm;

#[feature(asm_const)]
fn main() {
    let i: u64 = 3;
    let o: u64;
    let number: u64 = 5;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, {number}",
            out(reg) o,
            in(reg) i,
            number(reg) number,
        );
    }
    println!("{o}");
}
