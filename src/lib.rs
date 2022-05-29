struct MyIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let (element, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(element)

        // let element = self.slice.get(0);
        // self.slice = &self.slice[1..];
        // element
    }
}

struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice1 = &mut self.slice;
        let slice2 = std::mem::replace(slice1, &mut []);
        let (first, rest) = slice2.split_first_mut()?;
        self.slice = rest;
        Some(first)

        // // get first element
        // let element = self.slice.get_mut(0);
        // // set self.slice to the rest of the list
        // // self.slice = &mut self.slice[1..];
        // // return first element
        // element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyIterator {
            slice: &collection[..],
        };

        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }

        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };

        for (index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }

        assert_eq!(collection.get(0), Some(&2));
    }
}
