use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Cutoff<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    anim: A,
    cutoff: Duration,
    _marker: PhantomData<(V, C)>,
}

impl<A, V, C> Animation<V, C> for Cutoff<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim.sample(if elapsed < self.cutoff {
            elapsed
        } else {
            self.cutoff
        })
    }
}

impl<A, V, C> BoundedAnimation<V, C> for Cutoff<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn duration(&self) -> Duration {
        self.cutoff
    }
}

impl<A, V, C> Cutoff<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(anim: A, cutoff: Duration) -> Self {
        Self {
            anim,
            cutoff,
            _marker: PhantomData,
        }
    }
}
