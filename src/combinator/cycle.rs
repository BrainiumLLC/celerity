use crate::{Animatable, Animation, BoundedAnimation};
use std::marker::PhantomData;
use time_point::Duration;

#[derive(Debug)]
pub struct Cycle<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    anim: A,
    _marker: PhantomData<V>,
}

impl<A, V> Animation<V> for Cycle<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        let progress = elapsed.as_secs_f64() % self.anim.duration().as_secs_f64();
        self.anim.sample(Duration::from_secs_f64(progress))
    }
}

impl<A, V> Cycle<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    pub(crate) fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
