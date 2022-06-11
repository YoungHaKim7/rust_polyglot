trait Futures {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

impl<T> Poll<T> {
    pub fn map<U, F>(self, f: F) -> Poll<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(t) => Poll::Ready(f(t)),
            Poll::Pending => Poll::Pending,
        }
    }
}

enum Poll<T> {
    Ready(T),
    Pending,
}

fn my_function() // -> impl Futures<Output = ()>
{
    println!("I'm an async function!");
}

fn main() {
    my_function();
}

