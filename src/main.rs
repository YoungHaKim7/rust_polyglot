#[derive(Debug)]
struct Pair(i32, i32);

trait Coordinate {
    type A;
    type B;

    fn x(&self) -> i32; 
    fn y(&self) -> i32; 
}

impl Coordinate for Pair {
    type A = i32;
    type B = i32;

    fn x(&self) -> i32{self.0} 
    fn y(&self) -> i32{self.1} 
}

fn y_diff<C:Coordinate>(coor_1: &C, coor_2: &C) -> i32 {
    coor_2.y() - coor_1.y()
}


fn main() {
    let coor_1 = Pair(1,2);
    println!("{coor_1:?}");
    let coor_2 = Pair(3,9);

    println!("{}", y_diff(&coor_1, &coor_2));
}
    
