enum Node<'a> {
    Empty,
    NonEmpty(u32, &'a Node<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = &Node::Empty;
        let list = Node::NonEmpty(1091, node);
    }
}
