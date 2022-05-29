struct MyIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (element, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(element)
    }
}

struct MyMutableIterator<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T> Iterator for MyMutableIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // get first element
        let element = self.slice.get_mut(0);
        // set self.slice to the rest of the list
        self.slice = &mut self.slice[1..];
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
        let wrapper = MyIterator {
            slice: &collection[..],
        };

        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }

        let collection = vec![1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut &collection[..],
        };

        for (index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }

        assert_eq!(collection.get(0), Some(&2));
    }
}
