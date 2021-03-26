use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

// Delay animation A by a duration

pub struct Delay<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    anim: A,
    delay: Duration,
    _marker: PhantomData<(V, C)>,
}

impl<A, V, C> Animation<V, C> for Delay<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim
            .sample((elapsed - self.delay).max(Duration::zero()))
    }
}

impl<A, V, C> BoundedAnimation<V, C> for Delay<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn duration(&self) -> Duration {
        self.delay + self.anim.duration()
    }
}

impl<A, V, C> Delay<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(anim: A, delay: Duration) -> Self {
        Self {
            anim,
            delay,
            _marker: PhantomData,
        }
    }
}
