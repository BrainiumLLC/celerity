use crate::{util::ZipMap as _, Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Rev<A, O, T>
where
    A: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    anim: A,
    _marker: PhantomData<(O, T)>,
}

impl<A, O, T> Animation<O, T> for Rev<A, O, T>
where
    A: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn sample(&mut self, elapsed: Duration) -> O {
        self.anim.sample(
            self.duration()
                .zip_map(elapsed, |dur, el| std::cmp::max(dur - el, 0)),
        )
    }
}

impl<A, O, T> BoundedAnimation<O, T> for Rev<A, O, T>
where
    A: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn duration(&self) -> Duration {
        self.anim.duration()
    }
}

impl<A, O, T> Rev<A, O, T>
where
    A: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    pub fn new(anim: A) -> Self {
        Self {
            anim,
            _marker: PhantomData,
        }
    }
}
