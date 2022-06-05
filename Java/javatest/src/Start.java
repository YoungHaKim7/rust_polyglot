import java.util.Optional;

class Start {
    public static void main(String[] args) {
        // 6) Option (v.s. Null, ~Option)
        Optional<Foo> foo = Optional.empty();

        // 7) Result (v.s. Exception)
        try {
            var result = Start.floorDivide(10, 0);
            System.out.println("result : " + result);
        } catch (Exception ex) {
            System.out.println("ERROR CAUGHT - " + ex);
        }
    }

    public static int floorDivide(float num, float by) {
        if (by == 0) {
            throw new RuntimeException("Cannot divide by 0");
        }
        return (int) Math.floor(num / by);
    }
}

class Foo {
}
