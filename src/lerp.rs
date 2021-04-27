use crate::util::{Map, ZipMap};
use gee::en::{self, Num as _};
use std::fmt::Debug;

pub fn lerp<C: en::Num>(a: C, b: C, factor: f64) -> C {
    // This uses 2 multiplications to be numerically stable! Woo!
    (a.to_f64() * (1.0 - factor) + b.to_f64() * factor).cast()
}

pub fn linear_value<V: Animatable>(p0: &V, p1: &V, t0: f64, t1: f64, t: f64) -> V {
    let d10 = t1 - t0;
    let dt0 = t - t0;
    let d1t = t1 - t;

    if d10 != 0.0 {
        p0.zip_map(*p1, |v0, v1| {
            (v0.to_f64() * (d1t / d10) + v1.to_f64() * (dt0 / d10)).cast()
        })
    } else {
        *p0
    }
}

pub trait Animatable: Copy + Debug + ZipMap {
    fn lerp(self, other: Self, factor: f64) -> Self {
        self.zip_map(other, |a, b| lerp(a, b, factor))
    }

    // The shortest distance between two animatables (never negative)
    fn distance_to(self, other: Self) -> f64;
}

pub trait Scalar: en::Num {}

impl<S: Scalar> Map for S {
    type Component = Self;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        f(self)
    }
}

impl<S: Scalar> ZipMap for S {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        f(self, other)
    }
}

impl<S: Scalar> Animatable for S {
    fn distance_to(self, other: Self) -> f64 {
        (self - other).to_f64().abs()
    }
}

impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for u128 {}
impl Scalar for usize {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
impl Scalar for isize {}

impl<C: en::Num> Map for (C, C) {
    type Component = C;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        (f(self.0), f(self.1))
    }
}

impl<C: en::Num> ZipMap for (C, C) {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        (f(self.0, other.0), f(self.1, other.1))
    }
}

impl<C: en::Num> Animatable for (C, C) {
    fn distance_to(self, other: Self) -> f64 {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        (a * a + b * b).to_f64().sqrt()
    }
}

impl<C: en::Num> Map for gee::Point<C> {
    type Component = C;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        Self::from_tuple(self.to_tuple().map(f))
    }
}

impl<C: en::Num> ZipMap for gee::Point<C> {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        Self::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}

impl<C: en::Num> Animatable for gee::Point<C> {
    fn distance_to(self, other: Self) -> f64 {
        (self.to_tuple()).distance_to(other.to_tuple())
    }
}

impl<C: en::Num> Map for gee::Size<C> {
    type Component = C;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        Self::from_tuple(self.to_tuple().map(f))
    }
}

impl<C: en::Num> ZipMap for gee::Size<C> {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        Self::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}

impl<C: en::Num> Animatable for gee::Size<C> {
    fn distance_to(self, other: Self) -> f64 {
        (self.to_tuple()).distance_to(other.to_tuple())
    }
}

impl<C: en::Num> Map for gee::Vector<C> {
    type Component = C;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        Self::from_tuple(self.to_tuple().map(f))
    }
}

impl<C: en::Num> ZipMap for gee::Vector<C> {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        Self::from_tuple(self.to_tuple().zip_map(other.to_tuple(), f))
    }
}

impl<C: en::Num> Animatable for gee::Vector<C> {
    fn distance_to(self, other: Self) -> f64 {
        (self.to_tuple()).distance_to(other.to_tuple())
    }
}

impl<C: en::Float> Map for gee::Angle<C> {
    type Component = C;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        self.map_radians(f)
    }
}

impl<C: en::Float> ZipMap for gee::Angle<C> {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        self.map(|a| f(a, other.radians()))
    }
}

impl<C: en::Float> Animatable for gee::Angle<C> {
    fn distance_to(self, other: Self) -> f64 {
        // TODO: actually think about this
        (other.normalize().radians() - self.normalize().radians()).to_f64()
    }
}

impl Map for rainbow::LinRgba {
    type Component = f64;

    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component,
    {
        Self::from_f32_array(rainbow::util::map_all(self.into_f32_array(), |c| {
            // TODO: this is concerning
            f(c.cast()).to_f32()
        }))
    }
}

impl ZipMap for rainbow::LinRgba {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component,
    {
        let [ar, ag, ab, aa] = self.into_f32_array();
        let [br, bg, bb, ba] = other.into_f32_array();
        Self::from_f32(
            // TODO: this is even more concerning
            f(ar.cast(), br.cast()).to_f32(),
            f(ag.cast(), bg.cast()).to_f32(),
            f(ab.cast(), bb.cast()).to_f32(),
            f(aa.cast(), ba.cast()).to_f32(),
        )
    }
}

impl Animatable for rainbow::LinRgba {
    // TODO: have someone who actually understands math check this
    fn distance_to(self, other: Self) -> f64 {
        let [ar, ag, ab, aa] = self.into_f32_array();
        let [br, bg, bb, ba] = other.into_f32_array();
        let r = ar - br;
        let g = ag - bg;
        let b = ab - bb;
        let a = aa - ba;
        (r * r + g * g + b * b + a * a).to_f64().sqrt()
    }
}
