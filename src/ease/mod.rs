use crate::Animatable;
use gee::en;

pub fn eased_lerp<V, T>(a: V, b: V, f: T, easing_fn: impl EasingFn<T>) -> V
where
    V: Animatable<T>,
    T: en::Float,
{
    a.lerp(b, easing_fn.ease(f))
}

pub trait EasingFn<T: en::Num>: Copy {
    fn ease(&self, f: T) -> T;
}

impl<T: en::Num, F: Copy + Fn(T) -> T> EasingFn<T> for F {
    fn ease(&self, f: T) -> T {
        (*self)(f)
    }
}

pub fn cosine<T: en::Float>(f: T) -> T {
    let half = T::one().halved();
    half - half * T::cos(T::PI() * f)
}

pub fn half<T: en::Float>(f: T) -> T {
    f.powi(2)
}

pub fn slow_start<T: en::Float>(f: T) -> T {
    if f < T::one() {
        half(f)
    } else {
        T::two() * f - T::one()
    }
}

pub fn sine_ease_in<T: en::Float>(f: T) -> T {
    T::one() - T::cos((f * T::PI()) / T::two())
}

pub fn sine_ease_out<T: en::Float>(f: T) -> T {
    T::sin((f * T::PI()) / T::two())
}

pub fn sine_ease_in_out<T: en::Float>(f: T) -> T {
    -(T::cos(T::PI() * f) - T::one()) / T::two()
}

