use crate::{Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

pub struct Chain<A, B, V, T>
where
    A: BoundedAnimation<V, T>,
    B: Animation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    a: A,
    b: B,
    _marker: PhantomData<(V, T)>,
}

impl<A, B, V, T> Animation<V, T> for Chain<A, B, V, T>
where
    A: BoundedAnimation<V, T>,
    B: Animation<V, T>,
    V: Animatable<T>,
    T: en::Float,
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

impl<A, B, V, T> BoundedAnimation<V, T> for Chain<A, B, V, T>
where
    A: BoundedAnimation<V, T>,
    B: BoundedAnimation<V, T>,
    V: Animatable<T>,
    T: en::Float,
{
    fn duration(&self) -> Duration {
        self.a.duration() + self.b.duration()
    }
}

impl<A, B, V, T> Chain<A, B, V, T>
where
    A: BoundedAnimation<V, T>,
    B: Animation<V, T>,
    V: Animatable<T>,
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
