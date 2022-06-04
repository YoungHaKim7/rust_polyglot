fn sum_pos(v: &[i32]) -> i32 {
    let mut sum = 0;
    for i in v {
        if *i > 0 {
            sum += *i;
        }
    }
    sum
}

fn main() {
    let i = [0];
    let v1_iter = sum_pos(&i);

    for val in v1_iter..100 {
        println!("Got : {:?}", val);
    }
}

