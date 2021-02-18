use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Chain<A, B, V, C>
where
    A: BoundedAnimation<V, C>,
    B: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    a: A,
    b: B,
    _marker: PhantomData<(V, C)>,
}

impl<A, B, V, C> Animation<V, C> for Chain<A, B, V, C>
where
    A: BoundedAnimation<V, C>,
    B: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
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

impl<A, B, V, C> BoundedAnimation<V, C> for Chain<A, B, V, C>
where
    A: BoundedAnimation<V, C>,
    B: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn duration(&self) -> Duration {
        self.a.duration() + self.b.duration()
    }
}

impl<A, B, V, C> Chain<A, B, V, C>
where
    A: BoundedAnimation<V, C>,
    B: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _marker: PhantomData,
        }
    }
}
