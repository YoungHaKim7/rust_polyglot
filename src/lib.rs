struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (element, rest) = self.slice.split_first()?;
        // get the first element
        let element = self.slice.get(0);
        // set self.slice equal to the other elements
        self.slice = &self.slice[1..];
        // return first element
        element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let collection = vec![1, 2, 3, 4];
        let wrapper = MyIterWrapper {
            slice: &collection[..],
        };

        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }
    }
}
