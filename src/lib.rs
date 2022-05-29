struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // get the first element
        let element = self.slice.get(0);
        // set the other elements as self.slice
        // return first element
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyIterWrapper {
            slice: &collection[..],
        };

        for elem in wrapper {}
    }
}
