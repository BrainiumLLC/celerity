use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Blend<A, B, F, V, C>
where
    A: Animation<V, C>,
    B: Animation<V, C>,
    F: Animation<f64, f64>,
    V: Animatable<C>,
    C: en::Num,
{
    a: A,
    b: B,
    f: F,
    _marker: PhantomData<(V, C)>,
}

impl<A, B, F, V, C> Animation<V, C> for Blend<A, B, F, V, C>
where
    A: Animation<V, C>,
    B: Animation<V, C>,
    F: Animation<f64, f64>,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        let a_value = self.a.sample(elapsed);
        let b_value = self.b.sample(elapsed);
        let ratio = self.f.sample(elapsed);
        a_value.lerp(b_value, ratio)
    }
}

impl<A, B, F, V, C> BoundedAnimation<V, C> for Blend<A, B, F, V, C>
where
    A: BoundedAnimation<V, C>,
    B: BoundedAnimation<V, C>,
    F: Animation<f64, f64>,
    V: Animatable<C>,
    C: en::Num,
{
    fn duration(&self) -> Duration {
        self.a.duration().max(self.b.duration())
    }
}

impl<A, B, F, V, C> Blend<A, B, F, V, C>
where
    A: Animation<V, C>,
    B: Animation<V, C>,
    F: Animation<f64, f64>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(a: A, b: B, f: F) -> Self {
        Self {
            a,
            b,
            f,
            _marker: PhantomData,
        }
    }
}
