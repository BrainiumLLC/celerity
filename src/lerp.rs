use gee::en;
use std::fmt::Debug;

use crate::util::{Map, ZipMap};

pub fn lerp<C: en::Num>(a: C, b: C, factor: f64) -> C {
    // This uses 2 multiplications to be numerically stable! Woo!
    en::cast(en::cast::<f64, _>(a) * (1f64 - factor) + en::cast::<f64, _>(b) * factor)
}

pub fn linear_value<V: Animatable<C>, C: en::Num>(p0: &V, p1: &V, t0: f64, t1: f64, t: f64) -> V {
    let d10 = t1 - t0;
    let dt0 = t - t0;
    let d1t = t1 - t;

    if d10 != 0.0 {
        p0.zip_map(*p1, |v0, v1| {
            en::cast(en::cast::<f64, _>(v0) * (d1t / d10) + en::cast::<f64, _>(v1) * (dt0 / d10))
        })
    } else {
        *p0
    }
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
    fn distance_to(self, other: Self) -> f64 {
        (self.to_tuple()).distance_to(other.to_tuple())
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
        Self::from_tuple(self.to_tuple().map(f))
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
        Self::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}

impl<C> Animatable<C> for gee::Size<C>
where
    C: en::Num,
{
    fn distance_to(self, other: Self) -> f64 {
        (self.to_tuple()).distance_to(other.to_tuple())
    }
}

impl<C> Map<C> for gee::Size<C>
where
    C: en::Num,
{
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        Self::from_tuple(self.to_tuple().map(f))
    }
}

impl<C> ZipMap<C> for gee::Size<C>
where
    C: en::Num,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        Self::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}

impl<C> Animatable<C> for gee::Vector<C>
where
    C: en::Num,
{
    fn distance_to(self, other: Self) -> f64 {
        (self.to_tuple()).distance_to(other.to_tuple())
    }
}

impl<C> Map<C> for gee::Vector<C>
where
    C: en::Num,
{
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        Self::from_tuple(self.to_tuple().map(f))
    }
}

impl<C> ZipMap<C> for gee::Vector<C>
where
    C: en::Num,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        Self::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}

impl<C> Animatable<C> for gee::Angle<C>
where
    C: en::Float,
{
    fn distance_to(self, other: Self) -> f64 {
        // TODO: actually think about this
        en::cast(other.normalize().radians() - self.normalize().radians())
    }
}

impl<C> Map<C> for gee::Angle<C>
where
    C: en::Float,
{
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(C) -> C,
    {
        self.map_radians(f)
    }
}

impl<C> ZipMap<C> for gee::Angle<C>
where
    C: en::Float,
{
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(C, C) -> C,
    {
        self.map(|a| f(a, other.radians()))
    }
}

impl Animatable<f64> for rainbow::LinRgba {
    // TODO: have someone who actually understands math check this
    fn distance_to(self, other: Self) -> f64 {
        let [ar, ag, ab, aa] = self.into_f32_array();
        let [br, bg, bb, ba] = other.into_f32_array();
        let r = ar - br;
        let g = ag - bg;
        let b = ab - bb;
        let a = aa - ba;
        en::cast::<f64, _>(r * r + g * g + b * b + a * a).sqrt()
    }
}

impl Map<f64> for rainbow::LinRgba {
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self::from_f32_array(rainbow::util::map_all(self.into_f32_array(), |c| {
            // TODO: this is concerning
            en::cast(f(en::cast(c)))
        }))
    }
}

impl ZipMap<f64> for rainbow::LinRgba {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        let [ar, ag, ab, aa] = self.into_f32_array();
        let [br, bg, bb, ba] = other.into_f32_array();
        Self::from_f32(
            // TODO: this is even more concerning
            en::cast(f(en::cast(ar), en::cast(br))),
            en::cast(f(en::cast(ag), en::cast(bg))),
            en::cast(f(en::cast(ab), en::cast(bb))),
            en::cast(f(en::cast(aa), en::cast(ba))),
        )
    }
}
