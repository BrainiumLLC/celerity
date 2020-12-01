use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

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
    fn sample(&mut self, elapsed: Duration) -> O {
        let inflection = self.a.duration();
        if elapsed < inflection {
            self.a.sample(elapsed)
        } else {
            self.b.sample(elapsed - inflection)
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
