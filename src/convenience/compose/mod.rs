pub trait Compose<X> {
    fn compose(self) -> impl Fn(X) -> X;
}

impl<T: Iterator + Clone + 'static, X> Compose<X> for T
where
    <T as Iterator>::Item: Fn(X) -> X,
{
    /// Composes a series of operations `f0, f1, ..., fn` into single operation `f`.
    ///
    /// The operations are applied left-to-right, i.e. `f(x) = fn( .. f1( f0(x) ) .. )`.
    fn compose(self) -> impl Fn(X) -> X {
        // CAUTION: Cloning is absolutely crucial here.
        // self is an Iterator that is moved into the closure and therefore
        // returned from the function. It is used whenever the closure is
        // called. If we do not clone the iterator, it will be empty after
        // the first call which changes the intended behavior. Besides,
        // cloning an Iterator is cheap since it is merely a view into the
        // underlying data structure. The data structure itself is not
        // cloned.
        move |x| self.clone().fold(x, |x, f| f(x))
    }
}
