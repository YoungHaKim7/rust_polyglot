struct Foo {
    x: i32,
}

fn do_stuff(foo: &mut Foo) {
    foo.x = foo.x + 1;
    println!("do stuff: {}", foo.x);
}

fn main() {
    let mut foo = Foo { x: 123 };
    let y = foo.x;

    let foo_b = &mut foo;

    do_stuff(foo_b);
    do_stuff(&mut foo);
}

