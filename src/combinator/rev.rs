use crate::{util::ZipMap as _, Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Rev<A, V, T>
where
    A: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    anim: A,
    _marker: PhantomData<(V, T)>,
}

impl<A, V, T> Animation<V, T> for Rev<A, V, T>
where
    A: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim.sample(
            self.duration()
                .zip_map(elapsed, |dur, el| std::cmp::max(dur - el, 0)),
        )
    }
}

impl<A, V, T> BoundedAnimation<V, T> for Rev<A, V, T>
where
    A: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    fn duration(&self) -> Duration {
        self.anim.duration()
    }
}

impl<A, V, T> Rev<A, V, T>
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
