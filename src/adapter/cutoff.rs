use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::{Duration, TimePoint};

pub struct Cutoff<A, O, T>
where
    A: Animation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    anim: A,
    cutoff: Duration,
    _marker: PhantomData<(O, T)>,
}

impl<A, O, T> Animation<O, T> for Cutoff<A, O, T>
where
    A: Animation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn sample(&self, start: TimePoint, time: TimePoint) -> O {
        assert_start_lte_time!(Cutoff, start, time);
        let cutoff = start + self.cutoff;
        let time = if time < cutoff { time } else { cutoff };
        self.anim.sample(start, time)
    }
}

impl<A, O, T> BoundedAnimation<O, T> for Cutoff<A, O, T>
where
    A: Animation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn duration(&self) -> Duration {
        self.cutoff
    }
}

impl<A, O, T> Cutoff<A, O, T>
where
    A: Animation<O, T>,
    O: Output<T>,
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
