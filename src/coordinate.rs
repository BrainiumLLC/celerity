use std::fmt::Debug;

use gee::en;

use crate::{
    util::{Map, ZipMap},
    Animatable,
};

#[derive(Debug, Clone, Copy)]
// X & Y could be any combination of integers and floats
pub struct Coordinate<X: en::Num, Y: en::Num> {
    pub x: X, // Could be Duration or some other scalar
    pub y: Y, // Could be any scalar type
}

impl<X: en::Num, Y: en::Num> Coordinate<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }

    // Regardless of whether integers or floats are used in the calculation, our distance should be f64
    pub fn distance_to(&self, other: &Coordinate<X, Y>) -> f64 {
        f64::sqrt(
            f64::powi(en::cast(other.x - self.x), 2) + f64::powi(en::cast(other.y - self.y), 2),
        )
    }
}

impl<X: en::Num, Y: en::Num> std::ops::Add for Coordinate<X, Y> {
    type Output = Coordinate<X, Y>;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<X: en::Num, Y: en::Num> std::ops::Sub for Coordinate<X, Y> {
    type Output = Coordinate<X, Y>;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<X: en::Num, Y: en::Num> std::ops::Mul<f64> for Coordinate<X, Y> {
    type Output = Coordinate<X, Y>;

    fn mul(self, rhs: f64) -> Self {
        Self::new(
            self.x * en::cast::<X, f64>(rhs),
            self.y * en::cast::<Y, f64>(rhs),
        )
    }
}

// impl<X: en::Num, Y: en::Num> std::ops::Div for Coordinate<X, Y> {
//     type Output = Coordinate<X, Y>;

//     fn div(self, rhs: Self) -> Self {
//         Self::new(self.x / rhs.x, self.y / rhs.y)
//     }
// }

impl<C, X, Y> Animatable<C> for Coordinate<X, Y>
where
    C: en::Num,
    X: en::Num,
    Y: en::Num,
{
    fn lerp(self, other: Self, f: f64) -> Self {
        Coordinate::new(self.x.lerp(other.x, f), self.y.lerp(other.y, f))
    }

    fn distance_to(self, other: Self) -> f64 {
        let a = self.x - other.x;
        let b = self.y - other.y;
        (en::cast::<f64, _>(a * a) + en::cast::<f64, _>(b * b)).sqrt()
    }
}

impl<C, X, Y> Map<C> for Coordinate<X, Y>
where
    C: en::Num,
    X: en::Num,
    Y: en::Num,
{
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        Coordinate::new(en::cast(f(en::cast(self.x))), en::cast(f(en::cast(self.y))))
    }
}

impl<C, X, Y> ZipMap<C> for Coordinate<X, Y>
where
    C: en::Num,
    X: en::Num,
    Y: en::Num,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        Coordinate::new(
            en::cast(f(en::cast(self.x), en::cast(other.x))),
            en::cast(f(en::cast(self.y), en::cast(other.y))),
        )
    }
}
