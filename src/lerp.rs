use gee::en::{self, num_traits::pow};
use std::fmt::Debug;

use crate::util::{Map, ZipMap};

pub fn lerp<C: en::Num>(a: C, b: C, factor: f64) -> C {
    // This uses 2 multiplications to be numerically stable! Woo!
    en::cast(en::cast::<f64, _>(a) * (1f64 - factor) + en::cast::<f64, _>(b) * factor)
}

pub trait Animatable<C>: Clone + Copy + Debug + ZipMap<C>
where
    C: en::Num,
{
    fn lerp(self, other: Self, factor: f64) -> Self {
        self.zip_map(other, |a, b| lerp(a, b, factor))
    }

    // The shortest distance between two animatables (never negative)
    fn distance_to(self, other: Self) -> f64;
}

impl<C: en::Num> Animatable<C> for C {
    fn distance_to(self, other: Self) -> f64 {
        en::cast::<f64, _>(self - other).abs()
    }
}

impl<C> Map<C> for C
where
    C: en::Num,
{
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        f(self)
    }
}

impl<C> ZipMap<C> for C
where
    C: en::Num,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        f(self, other)
    }
}

impl<C> Animatable<C> for (C, C)
where
    C: en::Num,
{
    fn lerp(self, other: Self, f: f64) -> Self {
        (self.0.lerp(other.0, f), self.1.lerp(other.1, f))
    }

    fn distance_to(self, other: Self) -> f64 {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        en::cast::<f64, _>(a * a + b * b).sqrt()
    }
}

impl<C> Map<C> for (C, C) {
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        (f(self.0), f(self.1))
    }
}

impl<C> ZipMap<C> for (C, C) {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        (f(self.0, other.0), f(self.1, other.1))
    }
}

impl<C> Animatable<C> for gee::Point<C>
where
    C: en::Num,
{
    fn lerp(self, other: Self, f: f64) -> Self {
        gee::Point::from_tuple(self.to_tuple().lerp(other.to_tuple(), f))
    }

    fn distance_to(self, other: Self) -> f64 {
        let a = self.x - other.x;
        let b = self.y - other.y;
        en::cast::<f64, _>(a * a + b * b).sqrt()
    }
}

impl<C> Map<C> for gee::Point<C>
where
    C: en::Num,
{
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        gee::Point::from_tuple((f(self.x), f(self.y)))
    }
}

impl<C> ZipMap<C> for gee::Point<C>
where
    C: en::Num,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        gee::Point::from_tuple((f(self.x, other.x), f(self.y, other.y)))
    }
}
