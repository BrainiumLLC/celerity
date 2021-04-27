use crate::{Animatable, Animation, BoundedAnimation};
use std::marker::PhantomData;
use time_point::Duration;

pub struct Cutoff<A, V>
where
    A: Animation<V>,
    V: Animatable,
{
    anim: A,
    cutoff: Duration,
    _marker: PhantomData<V>,
}

impl<A, V> Animation<V> for Cutoff<A, V>
where
    A: Animation<V>,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim.sample(if elapsed < self.cutoff {
            elapsed
        } else {
            self.cutoff
        })
    }
}

impl<A, V> BoundedAnimation<V> for Cutoff<A, V>
where
    A: Animation<V>,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        self.cutoff
    }
}

impl<A, V> Cutoff<A, V>
where
    A: Animation<V>,
    V: Animatable,
{
    pub fn new(anim: A, cutoff: Duration) -> Self {
        Self {
            anim,
            cutoff,
            _marker: PhantomData,
        }
    }
}
