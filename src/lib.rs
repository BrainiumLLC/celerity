pub mod catmullrom;
pub mod combinator;
mod coordinate;
pub mod ease;
mod lerp;
pub mod track;
mod util;

use std::fmt::Debug;

pub use self::{combinator::*, lerp::*, track::*};

use crate::util::Map as _;
use gee::en;
use time_point::{Duration, TimePoint};

pub trait Animation<V: Animatable<T>, T: en::Float> {
    fn sample(&self, elapsed: Duration) -> V;

    fn cutoff(self, duration: Duration) -> Cutoff<Self, V, T>
    where
        Self: Sized,
    {
        Cutoff::new(self, duration)
    }
}

impl<V: Animatable<T>, T: en::Float> Debug for dyn Animation<V, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animation") // TODO: Include something meaningful?
    }
}

pub trait BoundedAnimation<V: Animatable<T>, T: en::Float>: Animation<V, T> {
    fn duration(&self) -> Duration;

    /// The last time that this animation needs to be sampled at.
    fn end(&self, start: TimePoint) -> TimePoint {
        start + self.duration()
    }

    fn chain<B>(self, other: B) -> Chain<Self, B, V, T>
    where
        Self: Sized,
        B: Animation<V, T>,
    {
        Chain::new(self, other)
    }

    fn cycle(self) -> Cycle<Self, V, T>
    where
        Self: Sized,
    {
        Cycle::new(self)
    }

    fn repeat(self, times: u32) -> Cutoff<Cycle<Self, V, T>, V, T>
    where
        Self: Sized,
    {
        let times: i64 = en::cast(times);
        let duration = Duration::new(self.duration().nanos * times);
        Cutoff::new(Cycle::new(self), duration)
    }

    fn mirror(self) -> Chain<Self, Rev<Self, V, T>, V, T>
    where
        Self: Clone + Sized,
    {
        self.clone().chain(self.rev())
    }

    fn rev(self) -> Rev<Self, V, T>
    where
        Self: Sized,
    {
        Rev::new(self)
    }
}

impl<F, V, T> Animation<V, T> for F
where
    F: Fn(Duration) -> V,
    V: Animatable<T>,
    T: en::Float,
{
    fn sample(&self, elapsed: Duration) -> V {
        (*self)(elapsed)
    }
}
