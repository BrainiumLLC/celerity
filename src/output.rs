use crate::ease;
use gee::en;
use std::fmt::Debug;

pub trait Output<T: en::Float>: Clone + Copy + Debug + Sized {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(T, T) -> T + Copy;

    fn lerp(self, other: Self, f: T) -> Self {
        self.zip_map(other, |a, b| ease::lerp(a, b, f))
    }

    fn eased_lerp(self, other: Self, f: T, easing_fn: impl ease::EasingFn<T>) -> Self {
        self.zip_map(other, |a, b| ease::eased_lerp(a, b, f, easing_fn))
    }
}

impl<T> Output<T> for T
where
    T: en::Float,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(T, T) -> T + Copy,
    {
        f(self, other)
    }
}

impl<T> Output<T> for (T, T)
where
    T: en::Float,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(T, T) -> T + Copy,
    {
        (f(self.0, other.0), f(self.1, other.1))
    }
}

impl<T> Output<T> for gee::Point<T>
where
    T: en::Float,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(T, T) -> T + Copy,
    {
        gee::Point::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}
