import java.util.ArrayList;

class Start {
    public static void main(String[] args) {

        // 5) Generics, Option, Result
        var c1 = new Container<Integer>(123);
        var c2 = new Container<String>("Hello");

        System.out.println("compare 1 : " + c1.compareItem(999));
        System.out.println("compare 2 : " + c2.compareItem("zzz"));
        System.out.println("compare 3 : " + Start.compare(222, 111));
    }

    public static <T extends Comparable> int compare(T item1, T item2) {
        return item1.compareTo(item2);
    }
}

class Container<T extends Comparable> {
    T item;

    Container(T item) {
        this.item = item;
    }

    int compareItem(T otherItem) {
        return item.compareTo(otherItem);
    }
}
