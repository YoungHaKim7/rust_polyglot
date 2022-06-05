import java.util.ArrayList;

class Start {
    public static void main(String[] args) {
        // 4) enum
        Activity activity_now = new Sleeping(5);
        var message = switch (activity_now) {
            case Sleeping sl && sl.hours > 8 -> "Wake up!" + sl.hours;
            case Sleeping sl -> "shit";
            case Skiing sk -> "Awesome! " + sk.resort;
            case Coding c -> "Hopefully in Java!  ";
            default -> "unknown";
        };

        System.out.println("The message is : " + message);

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
