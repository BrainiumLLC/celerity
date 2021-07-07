use crate::{Animatable, Animation, BoundedAnimation};
use std::marker::PhantomData;
use time_point::Duration;

/// See [`BoundedAnimation::chain`] for details.
#[derive(Debug)]
pub struct Chain<B, A, V>
where
    B: BoundedAnimation<V>,
    A: Animation<V>,
    V: Animatable,
{
    a: B,
    b: A,
    _marker: PhantomData<V>,
}

impl<B, A, V> Animation<V> for Chain<B, A, V>
where
    B: BoundedAnimation<V>,
    A: Animation<V>,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        let inflection = self.a.duration();
        if elapsed < inflection {
            self.a.sample(elapsed)
        } else {
            self.b.sample(elapsed - inflection)
        }
    }
}

impl<A, B, V> BoundedAnimation<V> for Chain<B, A, V>
where
    B: BoundedAnimation<V>,
    A: BoundedAnimation<V>,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        self.a.duration() + self.b.duration()
    }
}

impl<B, A, V> Chain<B, A, V>
where
    B: BoundedAnimation<V>,
    A: Animation<V>,
    V: Animatable,
{
    pub(crate) fn new(a: B, b: A) -> Self {
        Self {
            a,
            b,
            _marker: PhantomData,
        }
    }
}
