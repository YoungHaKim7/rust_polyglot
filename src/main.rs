use std::arch::asm;
fn do_nothing() {
    #![feature(asm)]
    unsafe {
        asm!("NOP");
    }

    // asm!("NOP");
    // That would be invalid here, because we are no longer in an
    // unsafe block.
}

fn main() {
    do_nothing();
}
