// Formatting

// 8진수
use std::fmt;

struct Length(i32);

impl fmt::Octal for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::Octal::fmt(&val, f) // delegate to i32's implementation
    }
}
fn main() {
    let l = Length(9);
    println!("l as octal is: {l:o}");
    // 0o12  8 진수 표기법
    // = 9 decimal 10 진수
}
