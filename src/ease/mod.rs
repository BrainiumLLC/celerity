pub(crate) mod bezier;
pub(crate) mod spline;

use gee::en;

use crate::Animatable;

pub fn eased_lerp<V, C>(a: V, b: V, f: f64, easing_fn: impl EasingFn) -> V
where
    V: Animatable<C>,
    C: en::Num,
{
    a.lerp(b, easing_fn.ease(f))
}

pub trait EasingFn: Copy {
    fn ease(&self, f: f64) -> f64;
}

impl<F: Copy + Fn(f64) -> f64> EasingFn for F {
    fn ease(&self, f: f64) -> f64 {
        (*self)(f)
    }
}

pub fn cosine(f: f64) -> f64 {
    let half = 0.5f64;
    half - half * f64::cos(std::f64::consts::PI * f)
}

pub fn half(f: f64) -> f64 {
    f.powi(2)
}

pub fn slow_start(f: f64) -> f64 {
    if f < 1f64 {
        half(f)
    } else {
        2f64 * f - 1f64
    }
}

pub fn sine_ease_in(f: f64) -> f64 {
    1f64 - f64::cos((f * std::f64::consts::PI) / 2f64)
}

pub fn sine_ease_out(f: f64) -> f64 {
    f64::sin((f * std::f64::consts::PI) / 2f64)
}

pub fn sine_ease_in_out(f: f64) -> f64 {
    -(f64::cos(std::f64::consts::PI * f) - 1f64 / 2f64)
}
