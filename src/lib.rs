pub mod after_effects;
pub mod combinator;
pub mod interval;
pub mod interval_track;
mod lerp;
pub mod spline;
pub mod track;
mod util;

use std::fmt::Debug;

pub use self::{combinator::*, lerp::*, track::*};

use gee::en;
use time_point::{Duration, TimePoint};

pub trait Animation<V: Animatable> {
    fn sample(&self, elapsed: Duration) -> V;

    fn cutoff(self, duration: Duration) -> Cutoff<Self, V>
    where
        Self: Sized,
    {
        Cutoff::new(self, duration)
    }

    fn sample_path(&self, start: Duration, end: Duration, detail: usize) -> Vec<(f64, V)> {
        (0..detail)
            .map(|i| {
                let t = (i as f64) / (detail as f64);
                let time = start + (end - start) * t;
                (time.as_secs_f64(), self.sample(time))
            })
            .collect()
    }
}

impl<V: Animatable> Debug for dyn Animation<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Animation") // TODO: Include something meaningful?
    }
}

pub trait BoundedAnimation<V: Animatable>: Animation<V> {
    fn duration(&self) -> Duration;

    /// The last time that this animation needs to be sampled at.
    fn end(&self, start: TimePoint) -> TimePoint {
        start + self.duration()
    }

    fn chain<B>(self, other: B) -> Chain<Self, B, V>
    where
        Self: Sized,
        B: Animation<V>,
    {
        Chain::new(self, other)
    }

    fn cycle(self) -> Cycle<Self, V>
    where
        Self: Sized,
    {
        Cycle::new(self)
    }

    fn repeat(self, times: u32) -> Cutoff<Cycle<Self, V>, V>
    where
        Self: Sized,
    {
        let times: i64 = en::cast(times);
        let duration = Duration::new(self.duration().nanos * times);
        Cutoff::new(Cycle::new(self), duration)
    }

    fn mirror(self) -> Chain<Self, Rev<Self, V>, V>
    where
        Self: Clone + Sized,
    {
        self.clone().chain(self.rev())
    }

    fn rev(self) -> Rev<Self, V>
    where
        Self: Sized,
    {
        Rev::new(self)
    }
}

impl<V: Animatable> Debug for dyn BoundedAnimation<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BoundedAnimation") // TODO: Include something meaningful?
    }
}

impl<F, V> Animation<V> for F
where
    F: Fn(Duration) -> V,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        (*self)(elapsed)
    }
}
