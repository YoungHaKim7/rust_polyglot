// Dungeons and Dragons
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

impl Constitution for Elf {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

impl Constitution for Human {
    fn constitution_bonus(&self) -> u8 {
        0
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

    let my_human = Human {
        name: String::from("NellHuman"),
    };

    // Return 2
    my_dwarf.constitution_bonus();

    // Return 1
    dbg!(my_half_orc.constitution_bonus());

    // Return 0
    dbg!(my_elf.constitution_bonus());
    my_human.constitution_bonus();
}
