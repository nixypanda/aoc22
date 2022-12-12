use num::integer::lcm;
use std::iter::Iterator;

pub trait Lcm<T> {
    fn lcm(self) -> Option<T>;
}

impl<T, I> Lcm<T> for I
where
    I: Iterator<Item = T>,
    T: Copy + PartialOrd + num::Integer,
{
    fn lcm(self) -> Option<T> {
        self.reduce(|acc, x| lcm(acc, x))
    }
}
