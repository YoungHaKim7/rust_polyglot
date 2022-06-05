import java.util.ArrayList;

class Start {
    public static void main(String[] args) {
        // 4) enum
        Activity activity_now = new Sleeping(9);
        switch (activity_now) {
            case Sleeping sl -> System.out.println("Wake up!" + sl.hours);
            case Skiing sk -> System.out.println("Awesome! " + sk.resort);
            case Coding c -> System.out.println("Hopefully in Java!  ");
            default -> System.out.println("unknown");
        }

        // 5) Generics, Option, Result

    }
}

class Activity {
}

class Sleeping extends Activity {
    public int hours;

    public Sleeping(int hours) {
        this.hours = hours;
    }
}

class Skiing extends Activity {
    public String resort;

    Skiing(String resort) {
        this.resort = resort;
    }
}

class Coding extends Activity {
}
