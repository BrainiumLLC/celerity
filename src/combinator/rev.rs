use crate::{util::ZipMap as _, Animatable, Animation, BoundedAnimation};
use std::marker::PhantomData;
use time_point::Duration;

pub struct Rev<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    anim: A,
    _marker: PhantomData<V>,
}

impl<A, V> Animation<V> for Rev<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim.sample(
            self.duration()
                .zip_map(elapsed, |dur, el| std::cmp::max(dur - el, 0)),
        )
    }
}

impl<A, V> BoundedAnimation<V> for Rev<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        self.anim.duration()
    }
}

impl<A, V> Rev<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    pub fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
