use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Cycle<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    anim: A,
    _marker: PhantomData<(V, C)>,
}

impl<A, V, C> Animation<V, C> for Cycle<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        let progress = elapsed.as_secs_f64() % self.anim.duration().as_secs_f64();
        self.anim.sample(Duration::from_secs_f64(progress))
    }
}

impl<A, V, C> Cycle<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
