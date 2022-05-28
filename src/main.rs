// trait __ Dungeons And Dragons
// the focus of this talk it traits with D&D used as a metaphor
// Dwarf, Elf, Half-Orc, Human

struct Dwarf {
    name: String,
}

struct Elf {
    name: String,
}

struct HalfOrc {
    name: String,
}

struct Human {
    name: String,
}

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

    // Return 2
    my_dwarf.constitution_bonus();

    // Return 1
    my_half_orc.constitution_bonus();

    // Return 0(default)
    my_elf.constitution_bonus();
    my_human.constitution_bonus();
}
