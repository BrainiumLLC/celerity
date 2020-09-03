use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::{Duration, TimePoint};

pub struct Chain<A, B, O, T>
where
    A: BoundedAnimation<O, T>,
    B: Animation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    a: A,
    b: B,
    _marker: PhantomData<(O, T)>,
}

impl<A, B, O, T> Animation<O, T> for Chain<A, B, O, T>
where
    A: BoundedAnimation<O, T>,
    B: Animation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn sample(&mut self, start: TimePoint, time: TimePoint) -> O {
        assert_start_lte_time!(Chain, start, time);
        let inflection = self.a.end(start);
        if time < inflection {
            self.a.sample(start, time)
        } else {
            self.b.sample(inflection, time)
        }
    }
}

impl<A, B, O, T> BoundedAnimation<O, T> for Chain<A, B, O, T>
where
    A: BoundedAnimation<O, T>,
    B: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn duration(&self) -> Duration {
        self.a.duration() + self.b.duration()
    }
}

impl<A, B, O, T> Chain<A, B, O, T>
where
    A: BoundedAnimation<O, T>,
    B: Animation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _marker: PhantomData,
        }
    }
}
