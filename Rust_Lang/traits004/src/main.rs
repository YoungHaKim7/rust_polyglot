
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

impl Constitution for Elf {}
impl Constitution for Human {}

// The constitution bonus for a dwarf is 2
impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}

pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

pub trait Elvish {
    fn speak_elvish(&self) -> String {
        String::from("yes")
    }

    fn no_speak_elvish(&self) -> String {
        String::from("no")
}

}

impl Elvish for Elf {}
impl Elvish for HalfElf {}
impl Elvish for HalfOrc {}



fn main() {
    let my_dwarf = Dwarf {
        name: String::from("NellDwarf"),
    };

    let my_elf = Elf {
        name: String::from("NellElf"),
    };

    let my_half_orc = HalfOrc {
        name: String::from("NellHalfOrc"),
    };

    let my_half_elf = HalfElf {
        name: String::from("NellHalfElf"),
    };

    let my_human = Human {
        name: String::from("NellHuman"),
    };

    // Return 2
    my_dwarf.constitution_bonus();

    // Return 1
    dbg!(my_human.constitution_bonus());

    // Return 0
    dbg!(my_elf.constitution_bonus());
    my_human.constitution_bonus();


    // Return "yes"
    dbg!(my_elf.speak_elvish());

    // Return "yes"
    dbg!(my_half_elf.speak_elvish());


    // Return "no"
    dbg!(my_half_orc.no_speak_elvish());

}
