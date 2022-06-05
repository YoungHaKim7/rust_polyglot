#[derive(Debug, Clone, Copy)]
struct Foo {}

// tuple struct
struct Bar(u64, bool, Foo);

fn main() {
    // 1) Array
    let arr1: [Foo; 4] = [Foo {}; 4];

    // 2) Tuple (n/a)
    let tuple1 = Bar(123, true, Foo {});

    // 3) Vec ( ~ ArrayList)

    // 4) enum & match (enum & Java 1 Switch)

    // 5) Generics

    // 6) Option (v.s. null & java. util. Optional)

    // 7) Result (v.s Exception)
}
