struct MyIterWrapper {
    slice: &[T],
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let collection = vec![1, 2, 3, 4];
        for elem in collection.iter_mut() {}
    }
}
