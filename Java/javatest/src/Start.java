class Start {
    public static void main(String[] args) {
        var c1 = new Contact("John Doe", (short) 2005);
        System.out.println("Hello " + c1.card());
    }
}

class Contact {
    String fullName;
    short since;

    Contact(String fullName, short since) {
        this.fullName = fullName;
        this.since = since;
    }

    public String info() {
        return this.fullName + " since: " + this.since;
    }

    public String card() {
        return "BusinessCard: " + this.fullName;
    }
}

interface BusinessCard {
    public String card();
}
