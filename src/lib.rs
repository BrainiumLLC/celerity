//! Buttery smooth animation toolkit.

pub mod after_effects;
mod combinators;
mod component_wise;
pub mod constant;
pub mod debug;
pub mod function;
pub mod interval;
pub mod interval_track;
mod lerp;
pub mod spline;

pub use self::{combinators::*, component_wise::*, lerp::*};
use gee::en;
use std::fmt::Debug;
use time_point::{Duration, TimePoint};

/// A value parameterized over time.
///
/// `Animation` is a semigroup, which makes me sound smart. They can thus be
/// combined using all sorts of cool combinators, while still producing an
/// `Animation` on the other end.
///
/// Implementors should only implement [`Animation::sample`].
pub trait Animation<V: Animatable>: Debug {
    /// Samples the animation at the specified duration.
    ///
    /// `elapsed` is the duration since the "start" of the animation. Animations
    /// don't have a fixed start time; they merely start at an `elapsed` of
    /// zero, and end at an `elapsed` of infinity. You're thus free to pick any
    /// start time you'd like, and the animation is simply given how much time
    /// has passed since that start time.
    ///
    /// # Rules
    /// - Sampling at the same `elapsed` multiple times will always return the
    /// same value.
    /// - Sampling at an `elapsed` smaller than one sampled at previously is
    /// valid.
    /// - The result is unspecified if `elapsed` is negative.
    fn sample(&self, elapsed: Duration) -> V;

    /// Allows you to use combinators on a mutable reference to an animation.
    ///
    /// This is typically only useful if you're using trait objects.
    ///
    /// # Warning
    /// If `f` panics, then the program will abort.
    ///
    /// # Examples
    /// ```
    /// use celerity::{Animation as _, BoundedAnimation};
    ///
    /// struct Game {
    ///     anim: Box<dyn BoundedAnimation<f32>>,
    /// }
    ///
    /// impl Game {
    ///     fn reverse(&mut self) {
    ///         self.anim.replace_with(|anim| Box::new(anim.rev()));
    ///     }
    /// }
    /// ```
    fn replace_with<F>(&mut self, f: F)
    where
        Self: Sized,
        F: FnOnce(Self) -> Self,
    {
        replace_with::replace_with_or_abort(self, f)
    }

    /// Adapts this animation into a [`BoundedAnimation`] by snipping it at the
    /// specified duration.
    fn cutoff(self, duration: Duration) -> Cutoff<Self, V>
    where
        Self: Sized,
    {
        Cutoff::new(self, duration)
    }

    fn interrupt<A>(
        self,
        other: A,
        interrupt_t: Duration,
        transition_t: Duration,
    ) -> Interrupt<Cutoff<Self, V>, A, V>
    where
        Self: Sized,
        A: Animation<V>,
    {
        Interrupt::new(self.cutoff(interrupt_t), other, interrupt_t, transition_t)
    }
}

/// An [`Animation`] where the value stops changing after a known duration.
///
/// Implementors should only implement [`BoundedAnimation::duration`].
pub trait BoundedAnimation<V: Animatable>: Animation<V> {
    /// The duration this animation changes over.
    ///
    /// # Rules (in addition to the rules on [`Animation::sample`])
    /// - Sampling at an `elapsed` greater than this duration will return the
    /// same value as when sampling at this duration.
    fn duration(&self) -> Duration;

    /// The last time that this animation needs to be sampled at.
    fn end(&self, start: TimePoint) -> TimePoint {
        start + self.duration()
    }

    /// Appends another animation after this animation.
    ///
    /// If the other animation is also a `BoundedAnimation`, then the resulting
    /// animation is a `BoundedAnimation`.
    fn chain<B>(self, other: B) -> Chain<Self, B, V>
    where
        Self: Sized,
        B: Animation<V>,
    {
        Chain::new(self, other)
    }

    /// Cycles this animation forever.
    ///
    /// The resulting animation is no longer bounded.
    fn cycle(self) -> Cycle<Self, V>
    where
        Self: Sized,
    {
        Cycle::new(self)
    }

    /// Repeats this animation a specified number of times.
    fn repeat(self, times: u32) -> Cutoff<Cycle<Self, V>, V>
    where
        Self: Sized,
    {
        let times: i64 = en::cast(times);
        let duration = Duration::new(self.duration().nanos * times);
        Cutoff::new(Cycle::new(self), duration)
    }

    /// Reverses this animation.
    fn rev(self) -> Rev<Self, V>
    where
        Self: Sized,
    {
        Rev::new(self)
    }

    /// Chains this animation with its reverse.
    ///
    /// Going there and back again has never been easier.
    fn mirror(self) -> Chain<Self, Rev<Self, V>, V>
    where
        Self: Clone + Sized,
    {
        self.clone().chain(self.rev())
    }
}

// impl<F, V> Animation<V> for F
// where
//     F: Fn(Duration) -> V,
//     V: Animatable,
// {
//     fn sample(&self, elapsed: Duration) -> V {
//         (*self)(elapsed)
//     }
// }

impl<A, V> Animation<V> for Box<A>
where
    A: Animation<V> + ?Sized,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        A::sample(&*self, elapsed)
    }
}

impl<A, V> BoundedAnimation<V> for Box<A>
where
    A: BoundedAnimation<V> + ?Sized,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        A::duration(&*self)
    }
}
