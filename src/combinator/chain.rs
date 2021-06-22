use crate::{Animatable, Animation, BoundedAnimation};
use std::marker::PhantomData;
use time_point::Duration;

pub struct Chain<A, B, V>
where
    A: BoundedAnimation<V>,
    B: Animation<V>,
    V: Animatable,
{
    a: A,
    b: B,
    _marker: PhantomData<V>,
}

impl<A, B, V> Animation<V> for Chain<A, B, V>
where
    A: BoundedAnimation<V>,
    B: Animation<V>,
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

impl<A, B, V> BoundedAnimation<V> for Chain<A, B, V>
where
    A: BoundedAnimation<V>,
    B: BoundedAnimation<V>,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        self.a.duration() + self.b.duration()
    }
}

impl<A, B, V> Chain<A, B, V>
where
    A: BoundedAnimation<V>,
    B: Animation<V>,
    V: Animatable,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _marker: PhantomData,
        }
    }
}
