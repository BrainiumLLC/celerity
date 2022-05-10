use crate::{Animatable, Animation, BoundedAnimation};
use std::{marker::PhantomData, time::Duration};

/// See [`BoundedAnimation::rev`] for details.
#[derive(Debug)]
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
            (elapsed < self.duration())
                .then(|| self.duration() - elapsed)
                .unwrap_or_else(|| Duration::ZERO),
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
    pub(crate) fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
