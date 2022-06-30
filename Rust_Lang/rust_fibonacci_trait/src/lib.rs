use num_traits::PrimInt;
use std::fmt::Debug;
use std::mem;

pub trait Primitive: Copy + Clone + Debug + PartialEq + PrimInt {}
impl<T: Copy + Clone + Debug + PartialEq + PrimInt> Primitive for T {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fibonacci<T: Primitive> {
    curr: T,
    next: T,
}

impl<T> Fibonacci<T>
where
    T: Primitive,
{
    pub fn new() -> Self {
        Self {
            curr: T::zero(),
            next: T::one(),
        }
    }
}

impl<T> Default for Fibonacci<T>
where
    T: Primitive,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Primitive,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr.checked_add(&self.next)?;
        let prev = mem::replace(&mut self.curr, self.next);
        self.next = next;
        Some(prev)
    }
}
