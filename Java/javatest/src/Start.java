import java.util.ArrayList;

class Start {
    public static void main(String[] args) {
        // 1) Array
        Foo[] arr1 = new Foo[10];
        System.out.println("item: " + arr1[3]);

        // 2) Tuple (n/a)
        // Not in Lang (org.javatuples, Pair, Triplet, ...)

        // 3) Vec (ArrayList)
        var arr2 = new ArrayList<Foo>();
        arr2.add(new Foo());
        System.out.println("item: " + arr2.get(0));

        // 4) enum

        // 5) Generics, Option, Result

    }
}

class Foo {
}
