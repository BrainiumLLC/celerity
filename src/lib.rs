pub mod combinator;
pub mod interval;
mod lerp;
pub mod spline;
pub mod track;
mod util;

use std::fmt::Debug;
use std::marker::PhantomData;

pub use self::{combinator::*, lerp::*, track::*};

use gee::en;
use time_point::{Duration, TimePoint};

// Epsilon for default velocity implementation
// TODO: check if this is not too small in practice
const VELOCITY_STEP: f64 = 1e-5;

pub trait Animation<V: Animatable<C>, C: en::Num> {
    fn sample(&self, elapsed: Duration) -> V;

    fn delay(self, duration: Duration) -> Delay<Self, V, C>
    where
        Self: Sized,
    {
        Delay::new(self, duration)
    }

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

pub trait SmoothAnimation<V: Animatable<C>, C: en::Num>: Animation<V, C> {

    fn dt_sample(&self, elapsed: Duration) -> V {
        let dt = Duration::from_secs_f64(VELOCITY_STEP);
        let a = self.sample(elapsed - dt);
        let b = self.sample(elapsed + dt);
        a.zip_map(b, |a, b| {
            en::cast(en::cast::<f64, _>(b - a) / (2.0 * VELOCITY_STEP))
        })
    }
    
    //}
    //
    // - Does it make sense to split this up?
    // - Should dt_sample just be available on any Animation?
    //   -> getting analytical derivative in the presence of easing/rectification is hard
    //   -> who is going to bother to roll their own?
    //
    //pub trait InterruptibleAnimation<V: Animatable<C>, C: en::Num>: SmoothAnimation<V, C> {

    fn interrupt_at<A>(
        self,
        other: A,
        elapsed: Duration,
        transition: Duration,
    ) -> Blend<Linear<V, C>, Self, CosineEase, V, C>
    where
        A: SmoothAnimation<V, C>,
        Self: Sized,
    {
        // Sample position and velocity at interruption time
        let value = other.sample(elapsed);
        let dt_value = other.dt_sample(elapsed);

        // Create linear continuation
        let linear = Linear::new(value, dt_value);
        // Fn(Duration) -> V
        // let linear = |elapsed| value + dt_value * elapsed.as_secs_f64();

        // Option A:
        // blend between ramp and B
        // lerp(linear(A), B, Cosine)
        // -> influence of B at start is 0

        // Create cosine ease
        let f = CosineEase::new(transition);
        // Fn(Duration) -> V
        // let f = |elapsed| 0.5 - 0.5 * (....).cos()

        // Return blend from linear to self
        Blend::new(linear, self, f)

        // Option B:
        // ramp to 0 + B
        // lerp(linear(A), 0, Cosine) + B
        // -> influence of B is always 100%
        
        // The difference:
        // - Option A is always smooth even if B does not start smoothly
        //   -> Option A alters B even if A is static
        // - Option B is only smooth if B starts smoothly
        //   -> Option B always gives faithful playback of B if A is static
        
        // Same Problem: return type is dramatically different for both
        // - Option A: Blend<Linear<...>, Self, ...>
        // - Option B: Sum<Blend<...>, Self, ...>
        
    }

    fn continue_from<A>(
        self,
        other: A,
        elapsed: Duration,
        transition: Duration,
    ) -> Delay<Blend<Linear<V, C>, Self, CosineEase, V, C>, V, C>
    where
        A: SmoothAnimation<V, C>,
        Self: Sized,
    {
        self.interrupt_at(other, elapsed, transition).delay(elapsed)
    }
}

/*
// Can't make interrupt_at generalized on the return type
// ... associated type refers to Self

impl<V, C> dyn SmoothAnimation<
    V,
    C,
    Interrupted = Blend<Linear<V, C>, Self, CosineEase, V, C>
>
where 
V: Animatable<C>,
C: en::Num,
{
    fn interrupt_at<A>(
        self,
        other: A,
        elapsed: Duration,
        transition: Duration,
    ) -> Self::Interrupted
    where
        A: SmoothAnimation<V, C>,
        Self: Sized,
    {
        // Sample position and velocity at interruption time
        let value = other.sample(elapsed);
        let dt_value = other.dt_sample(elapsed);

        // Create linear continuation
        let linear = Linear::new(value, dt_value);

        // Create cosine ease
        let f = CosineEase::new(transition);

        // Return blend
        Blend::new(linear, self, f)
    }

    fn continue_from<A>(
        self,
        other: A,
        elapsed: Duration,
        transition: Duration,
    ) -> Delay<Self::Interrupted, V, C>
    where
        A: SmoothAnimation<V, C>,
        Self: Sized,
    {
        self.interrupt_at(other, elapsed, transition).delay(elapsed)
    }
}
*/

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

pub struct Linear<V, C> {
    value: V,
    dt_value: V,
    _marker: PhantomData<C>,
}

impl<V, C> Linear<V, C>
where
    V: Animatable<C>,
    C: en::Num,
{
    fn new(value: V, dt_value: V) -> Self {
        Self { value, dt_value, _marker: PhantomData }
    }
}

impl<V, C> Animation<V, C> for Linear<V, C>
where
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.value.zip_map(self.dt_value, |v, dvdt| {
            v + dvdt * en::cast::<C, _>(elapsed.as_secs_f64())
        })
    }
}

pub struct CosineEase {
    duration: Duration,
}

impl CosineEase {
    fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl Animation<f64, f64> for CosineEase {
    fn sample(&self, elapsed: Duration) -> f64 {
        0.5 - 0.5
            * (elapsed
                .clamp(Duration::zero(), self.duration)
                .div_duration_f64(self.duration)
                * std::f64::consts::PI)
                .cos()
    }
}

impl BoundedAnimation<f64, f64> for CosineEase {
    fn duration(&self) -> Duration {
        self.duration
    }
}

