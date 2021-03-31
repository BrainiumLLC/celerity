pub mod combinator;
pub mod interval;
mod lerp;
pub mod spline;
pub mod track;
mod util;

use std::fmt::Debug;

pub use self::{combinator::*, lerp::*, track::*};

use gee::en;
use time_point::{Duration, TimePoint};

pub trait Animation<V: Animatable<C>, C: en::Num> {
    fn sample(&self, elapsed: Duration) -> V;

    fn cutoff(self, duration: Duration) -> Cutoff<Self, V, C>
    where
        Self: Sized,
    {
        Cutoff::new(self, duration)
    }
}

impl<V: Animatable<C>, C: en::Num> Debug for dyn Animation<V, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animation") // TODO: Include something meaningful?
    }
}

pub trait BoundedAnimation<V: Animatable<C>, C: en::Num>: Animation<V, C> {
    fn duration(&self) -> Duration;

    /// The last time that this animation needs to be sampled at.
    fn end(&self, start: TimePoint) -> TimePoint {
        start + self.duration()
    }

    fn chain<B>(self, other: B) -> Chain<Self, B, V, C>
    where
        Self: Sized,
        B: Animation<V, C>,
    {
        Chain::new(self, other)
    }

    fn cycle(self) -> Cycle<Self, V, C>
    where
        Self: Sized,
    {
        Cycle::new(self)
    }

    fn repeat(self, times: u32) -> Cutoff<Cycle<Self, V, C>, V, C>
    where
        Self: Sized,
    {
        let times: i64 = en::cast(times);
        let duration = Duration::new(self.duration().nanos * times);
        Cutoff::new(Cycle::new(self), duration)
    }

    fn mirror(self) -> Chain<Self, Rev<Self, V, C>, V, C>
    where
        Self: Clone + Sized,
    {
        self.clone().chain(self.rev())
    }

    fn rev(self) -> Rev<Self, V, C>
    where
        Self: Sized,
    {
        Rev::new(self)
    }
}

impl<V: Animatable<C>, C: en::Num> Debug for dyn BoundedAnimation<V, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BoundedAnimation") // TODO: Include something meaningful?
    }
}

impl<F, V, C> Animation<V, C> for F
where
    F: Fn(Duration) -> V,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        (*self)(elapsed)
    }
}
