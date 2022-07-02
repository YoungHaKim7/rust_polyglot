use std::arch::asm;
#[feature(asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn subtract(first: i32, second: i32) {
    unsafe {
        // Output values must either be unassigned (let result;) or mutable.
        let result: i32;
        // Each value that you pass in will be in a certain register, which
        // can be accessed with $0, $1, $2...
        //
        // The registers are assigned from left to right, so $0 is the
        // register containing 'result', $1 is the register containing
        // 'first' and $2 is the register containing 'second'.
        //
        // Rust uses AT&T syntax by default, so the format is:
        // SUB source, destination
        // which is equivalent to:
        // destination -= source;
        //
        // Because we want to subtract the first from the second,
        // we use the 0 constraint on 'first' to use the same
        // register as the output.
        // Therefore, we're doing:
        // SUB second, first
        // and getting the value of 'first'

        asm!("SUB $2, $0" : "=r"(result) : "0"(first), "r"(second));
        println!("{}", result);
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn subtract_mac(first: i32, second: i32) {
    unsafe {
        let result_mac: i32;

        asm!("SUB $2, $0" : "=r"(result) : "0"(first), "r"(second));
        println!("{}", result);
    }
}

fn main() {
    subtract_mac(2, 3);
}
