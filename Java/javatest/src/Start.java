import java.util.Optional;

class Start {
    public static void main(String[] args) {
        var foo = new Foo(123);

        var fooB = foo;

        Start.doStuff(foo);
        Start.doStuff(fooB);
    }

    static void doStuff(Foo foo) {
        foo.x = foo.x + 1;
        System.out.println("do stuff: " + foo.x);
    }
}

class Foo {
    int x;

    Foo(int x) {
        this.x = x;
    }
}
