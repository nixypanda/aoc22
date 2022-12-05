use std::ops::RangeInclusive;

pub trait Subsume<T> {
    fn subsumes(&self, other: &Self) -> bool;
}

impl<T> Subsume<T> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn subsumes(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
}

pub trait Overlap<T> {
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> Overlap<T> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }
}
