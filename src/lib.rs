mod combinator;
pub mod ease;
mod lerp;
mod track;
mod util;

pub use self::{combinator::*, lerp::*, track::*};

use crate::util::Map as _;
use gee::en;
use time_point::{Duration, TimePoint};

pub trait Animation<O: Output<T>, T: en::Float> {
    fn sample(&mut self, elapsed: Duration) -> O;

    fn cutoff(self, duration: Duration) -> Cutoff<Self, O, T>
    where
        Self: Sized,
    {
        Cutoff::new(self, duration)
    }
}

pub trait BoundedAnimation<O: Output<T>, T: en::Float>: Animation<O, T> {
    fn duration(&self) -> Duration;

    /// The last time that this animation needs to be sampled at.
    fn end(&self, start: TimePoint) -> TimePoint {
        start + self.duration()
    }

    fn chain<B>(self, other: B) -> Chain<Self, B, O, T>
    where
        Self: Sized,
        B: Animation<O, T>,
    {
        Chain::new(self, other)
    }

    fn cycle(self) -> Cycle<Self, O, T>
    where
        Self: Sized,
    {
        Cycle::new(self)
    }

    fn repeat(self, times: u32) -> Cutoff<Cycle<Self, O, T>, O, T>
    where
        Self: Sized,
    {
        let times: i64 = en::cast(times);
        let duration = self.duration().map(|nanos| nanos * times);
        Cutoff::new(Cycle::new(self), duration)
    }

    fn mirror(self) -> Chain<Self, Rev<Self, O, T>, O, T>
    where
        Self: Clone + Sized,
    {
        self.clone().chain(self.rev())
    }

    fn rev(self) -> Rev<Self, O, T>
    where
        Self: Sized,
    {
        Rev::new(self)
    }
}

impl<F, O, T> Animation<O, T> for F
where
    F: Fn(Duration) -> O,
    O: Output<T>,
    T: en::Float,
{
    fn sample(&mut self, elapsed: Duration) -> O {
        (*self)(elapsed)
    }
}
