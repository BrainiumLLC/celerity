use crate::{util::ZipMap as _, Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Rev<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    anim: A,
    _marker: PhantomData<(V, C)>,
}

impl<A, V, C> Animation<V, C> for Rev<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.anim.sample(
            self.duration()
                .zip_map(elapsed, |dur, el| std::cmp::max(dur - el, 0)),
        )
    }
}

impl<A, V, C> BoundedAnimation<V, C> for Rev<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn duration(&self) -> Duration {
        self.anim.duration()
    }
}

impl<A, V, C> Rev<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
