fn sum_pos(v: &[i32]) -> i32 {
    v.iter().filter(|i| **i > 0).map(|i| *i).sum()
}

fn main() {
    let i = [0];
    let v1_iter = sum_pos(&i);

    for val in v1_iter..100 {
        println!("Got : {:?}", val);
    }
}

