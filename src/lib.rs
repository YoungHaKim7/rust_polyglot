// struct MyIterWrapper {
//     slice: &[T],
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let collection = &mut [1, 2, 3, 4];
        for elem in collection.iter_mut() {
            *elem += 2;
        }
        assert_eq!(collection, &[3, 4, 5, 6]);
    }
}
