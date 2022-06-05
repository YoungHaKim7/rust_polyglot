use std::cmp::{Ord, Ordering};

fn compare_item<T: Ord>(item1: &T, item2: &T) -> Ordering {
    item1.cmp(item2)
}

struct Container<T: Ord> {
    item: T,
}

impl<T: Ord> Container<T> {
    fn compare_item(&self, other_item: &T) -> Ordering {
        self.item.cmp(other_item)
    }
}

fn main() {
    // 5) Generics
    let c1 = Container { item: 123 };
    let c2 = Container {
        item: "Hello".to_string(),
    };

    println!("compare 1 : {:?}", c1.compare_item(&222));
    println!("compare 2 : {:?}", c2.compare_item(&"zzzz".to_string()));
    println!("compare 3 : {:?}", compare_item(&222, &111));

    // 6) Option (v.s. null & java. util. Optional)

    // 7) Result (v.s Exception)
}
