pub trait FirstAndLast {
    type Item: Copy;

    /// Consumes the iterator, returning its first and last items.
    /// If the iterator only has one item, they will be the same.
    /// If it is empty, returns None.
    fn first_and_last(self) -> Option<(Self::Item, Self::Item)>;
}

impl<I, T> FirstAndLast for I
where
    T: Copy,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn first_and_last(mut self) -> Option<(Self::Item, Self::Item)> {
        let first = self.next();
        let last = self.last().or(first);
        first.zip(last)
    }
}
