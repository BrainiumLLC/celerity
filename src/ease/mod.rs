use gee::en;

pub fn lerp<T: en::Float>(a: T, b: T, f: T) -> T {
    a + (b - a) * f
}

pub fn eased_lerp<T: en::Float>(a: T, b: T, f: T, easing_fn: impl EasingFn<T>) -> T {
    easing_fn.lerp(a, b, f)
}

pub trait EasingFn<T: en::Float>: Copy {
    fn ease(&self, f: T) -> T;

    fn lerp(&self, a: T, b: T, f: T) -> T {
        lerp(a, b, self.ease(f))
    }
}

impl<T: en::Float, F: Copy + Fn(T) -> T> EasingFn<T> for F {
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
