#[derive(Debug, Clone, Copy)]
struct Foo {}

enum Activity {
    Sleeping(u8),   // number of hours slept
    Skiing(String), // ski resort
    Coding,
}

fn main() {
    // 4) enum & match (enum & Java 1 Switch)
    let activity_now = Activity::Sleeping(9);
    let message = match activity_now {
        Activity::Sleeping(hours) if hours > 8 => format!("Wake up!! {} o'clock", hours),
        Activity::Sleeping(_) => format!("shit!!"),
        Activity::Skiing(resort) => format!("Awesome! {}", resort),
        Activity::Coding => format!("Hopefully in Rust!  "),
    };
    println!("The message is : {}", message);

    // 5) Generics

    // 6) Option (v.s. null & java. util. Optional)

    // 7) Result (v.s Exception)
}
