use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Cycle<A, V, T>
where
    A: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    anim: A,
    _marker: PhantomData<(V, T)>,
}

impl<A, V, T> Animation<V, T> for Cycle<A, V, T>
where
    A: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    fn sample(&self, elapsed: Duration) -> V {
        let progress = elapsed.as_secs_f64() % self.anim.duration().as_secs_f64();
        self.anim.sample(Duration::from_secs_f64(progress))
    }
}

impl<A, V, T> Cycle<A, V, T>
where
    A: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    pub fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
