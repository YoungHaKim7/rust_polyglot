// trait __ Dungeons And Dragons
// the focus of this talk it traits with D&D used as a metaphor
// Dwarf, Elf, Half-Orc, Human

struct Dwarf {
    name: String,
}

struct Elf {
    name: String,
}

struct HalfElf {
    name: String,
}

struct HalfOrc {
    name: String,
}

struct Human {
    name: String,
}

// All of these need to be cast
struct Cantrip {}

struct Enchantment {}

struct Transmutation {}

struct Necromancy {}

struct Spellbook {
    pub spells: Vec<Box<dyn Cast>>,
}

// Vector contains Boxes
// <T> <- Type of value Box can point to
// <Cast> <- Box : must point to a value the implement Cast
// Vec<T> <- Vector that contain object of a trait
// How can we cast ALL of these

// The constitution bonus for a dwarf is 2
impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

// The constitution bonus for a dwarf is 1
impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}

impl Constitution for Elf {}
impl Constitution for Human {}

// The constitution bonus for both a human and a half-elf is 0
// Let's make 0 the default
pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

pub trait Elvish {}

impl Spellbook {
    pub fn run(&self) {
        for spell in self.spells.iter() {
            spell.cast(); // Cast spell
        }
    }
}

pub trait Cast {
    fn cast(&self);
}

impl Cast for Cantrip {
    fn cast(&self) {
        // Details of casting a Cantrip Spell
    }
}

impl Cast for Transmutation {
    fn cast(&self) {
        // Details of casting a Transmutation Spell
    }
}

impl Cast for Enchantment {
    fn cast(&self) {
        // Details of casting a Enchantment Spell
    }
}

impl Cast for Necromancy {
    fn cast(&self) {
        // Details of casting a Necromancy Spell
    }
}

impl Elvish for Elf {}
impl Elvish for HalfElf {}
impl Elvish for HalfOrc {}

// Let's make a function for speaking Elvish
// Accept a generic type "T" (character: T)
// Only implement the Elvish Trait<T: Elvish>
pub fn speak_elvish<T: Elvish>(character: T) -> String {
    String::from("yes")
}

pub fn no_speak_elvish<T: Elvish>(character: T) -> String {
    String::from("no")
}

// you cannot add data to a trait object
// Trait objects contain both data(Pointer) and behavior(Trait)

// Spell in D&D (Cantrip, Transmutation, Enchantment, Necromancy)

// Trait objects are great for heterogeneous collections
// It doesn't matter what type something is, as long as it implements
// a certain trait

// Trait are hard...... at first
// Trait are awesome!
// I have learned to harness the power of traits... And so can you!!!

fn main() {
    let my_dwarf = Dwarf {
        name: String::from("NellDwarf"),
    };

    let my_half_orc = HalfOrc {
        name: String::from("NellHalfOrc"),
    };

    let my_elf = Elf {
        name: String::from("NellElf"),
    };

    let my_human = Human {
        name: String::from("Nell"),
    };

    let my_half_elf = HalfElf {
        name: String::from("NellElf"),
    };

    let spell_book = Spellbook {
        // Different types of spells (each implement Cast)
        spells: vec![
            Box::new(Cantrip {}),
            Box::new(Transmutation {}),
            Box::new(Enchantment {}),
            Box::new(Necromancy {}),
        ],
    };

    // Casts each spell
    spell_book.run();

    // Return 2
    my_dwarf.constitution_bonus();

    // Return 1
    my_half_orc.constitution_bonus();

    // Return 0(default)
    my_elf.constitution_bonus();
    my_human.constitution_bonus();

    // Return "yes"
    speak_elvish(my_elf);

    // Return "yes"
    speak_elvish(my_half_elf);

    // Return "no"
    no_speak_elvish(my_half_orc);
}
