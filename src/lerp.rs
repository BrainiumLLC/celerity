use gee::en;
use std::fmt::Debug;

pub trait Output<T: en::Num>: Clone + Copy + Debug + Sized {
    fn lerp(self, other: Self, f: T) -> Self;
}

impl<T> Output<T> for T
where
    T: en::Num,
{
    fn lerp(self, other: Self, f: T) -> Self {
        // This uses 2 multiplications to be numerically stable! Woo!
        self * (T::one() - f) + other * f
    }
}

impl<T> Output<T> for (T, T)
where
    T: en::Num,
{
    fn lerp(self, other: Self, f: T) -> Self {
        (self.0.lerp(other.0, f), self.1.lerp(other.1, f))
    }
}

impl<T> Output<T> for gee::Point<T>
where
    T: en::Num,
{
    fn lerp(self, other: Self, f: T) -> Self {
        gee::Point::from_tuple(self.to_tuple().lerp(other.to_tuple(), f))
    }
}
