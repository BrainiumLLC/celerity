use gee::en;
use time_point::Duration;

pub trait Map: Sized {
    type Component: en::Num;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component;
}

pub trait ZipMap: Map {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component;
}

impl Map for Duration {
    type Component = i64;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        Self::new(f(self.nanos))
    }
}

impl ZipMap for Duration {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        self.map(|nanos| f(nanos, other.nanos))
    }
}
