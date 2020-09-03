use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::{Duration, TimePoint};

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
    fn sample(&mut self, start: TimePoint, time: TimePoint) -> O {
        assert_start_lte_time!(Rev, start, time);
        let rev = self.end(start) - (time - start);
        self.anim
            // TODO: this special-casing is probably bad
            .sample(start, if rev >= start { rev } else { start })
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
