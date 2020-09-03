use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Cycle<A, O, T>
where
    A: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    anim: A,
    _marker: PhantomData<(O, T)>,
}

impl<A, O, T> Animation<O, T> for Cycle<A, O, T>
where
    A: BoundedAnimation<O, T>,
    O: Output<T>,
    T: en::Float,
{
    fn sample(&mut self, elapsed: Duration) -> O {
        let progress = elapsed.as_secs_f64() % self.anim.duration().as_secs_f64();
        self.anim.sample(Duration::from_secs_f64(progress))
    }
}

impl<A, O, T> Cycle<A, O, T>
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
