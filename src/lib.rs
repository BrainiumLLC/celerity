pub mod after_effects;
pub mod combinator;
pub mod interval;
pub mod interval_track;
mod lerp;
pub mod spline;
mod util;

pub use self::{combinator::*, lerp::*};
use gee::en::{self, Num as _};
use std::fmt::Debug;
use time_point::{Duration, TimePoint};

pub trait Animation<V: Animatable> {
    fn sample(&self, elapsed: Duration) -> V;

    fn cutoff(self, duration: Duration) -> Cutoff<Self, V>
    where
        Self: Sized,
    {
        Cutoff::new(self, duration)
    }

    fn debug_path(&self, sample_count: usize, sample_duration: Duration) -> Vec<V> {
        (0..sample_count)
            .map(|i| {
                self.sample(
                    sample_duration * (en::cast::<f64, _>(i) / en::cast::<f64, _>(sample_count)),
                )
            })
            .collect()
    }

    // Sampling error can occur arround tight curves, showing reduced velocity
    fn debug_velocity(&self, sample_count: usize, sample_duration: Duration) -> Vec<V> {
        let sample_delta = sample_duration.as_secs_f64() / sample_count.to_f64();
        self.debug_path(sample_count + 1, sample_duration)
            .windows(2)
            .map(|window| {
                window[1].zip_map(window[0], |a, b| {
                    (a - b) / V::cast_component::<f64>(sample_delta)
                })
            })
            .collect()
    }

    // Highly sensitive to sampling errors in velocity
    fn debug_acceleration(&self, sample_count: usize, sample_duration: Duration) -> Vec<V> {
        self.debug_velocity(sample_count + 1, sample_duration)
            .windows(2)
            .map(|window| window[1].zip_map(window[0], |a, b| a - b))
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
