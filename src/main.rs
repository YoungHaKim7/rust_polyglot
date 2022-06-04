use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("kr", "Korea");
    let k = "kr";
    let o = m.get(&k);
    m.clear();
    if let Some(v) = o {
        println!("{} = {} ", k, *v);
    }
}

