use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

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
    fn sample(&mut self, elapsed: Duration) -> O {
        self.anim.sample(if elapsed < self.cutoff {
            elapsed
        } else {
            self.cutoff
        })
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
