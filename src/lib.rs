struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut collection = vec![1, 2, 3, 4];
        for elem in collection.iter_mut() {}
    }
}
