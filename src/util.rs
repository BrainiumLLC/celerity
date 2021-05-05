use gee::en;
use time_point::Duration;

pub trait ComponentWise: Sized {
    type Component: en::Num;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component;

    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component;

    fn add(self, other: Self) -> Self {
        self.zip_map(other, std::ops::Add::add)
    }
    fn sub(self, other: Self) -> Self {
        self.zip_map(other, std::ops::Sub::sub)
    }
}

impl ComponentWise for Duration {
    type Component = i64;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        Self::new(f(self.nanos))
    }

    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        self.map(|nanos| f(nanos, other.nanos))
    }
}
