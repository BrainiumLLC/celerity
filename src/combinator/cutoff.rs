use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Cutoff<A, V, T>
where
    A: Animation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    anim: A,
    cutoff: Duration,
    _marker: PhantomData<(V, T)>,
}

impl<A, V, T> Animation<V, T> for Cutoff<A, V, T>
where
    A: Animation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim.sample(if elapsed < self.cutoff {
            elapsed
        } else {
            self.cutoff
        })
    }
}

impl<A, V, T> BoundedAnimation<V, T> for Cutoff<A, V, T>
where
    A: Animation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    fn duration(&self) -> Duration {
        self.cutoff
    }
}

impl<A, V, T> Cutoff<A, V, T>
where
    A: Animation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    pub fn new(anim: A, cutoff: Duration) -> Self {
        Self {
            anim,
            cutoff,
            _marker: PhantomData,
        }
    }
}
