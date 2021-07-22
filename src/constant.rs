use crate::{Animatable, Animation, BoundedAnimation};
use time_point::Duration;

/// An animation that never changes.
///
/// This likely isn't very useful, besides the potential as a default. Its main
/// purpose is to demonstrate the properties of the most fundamental animation
/// possible.
#[derive(Debug)]
#[repr(transparent)]
pub struct Constant<V>
where
    V: Animatable,
{
    value: V,
}

impl<V> Animation<V> for Constant<V>
where
    V: Animatable,
{
    fn sample(&self, _elapsed: Duration) -> V {
        self.value
    }
}

impl<V> BoundedAnimation<V> for Constant<V>
where
    V: Animatable,
{
    fn duration(&self) -> Duration {
        // The `duration` is the period over which the animation changes, and
        // since `Constant` never changes, this is just zero.
        Duration::zero()
    }
}

impl<V> Constant<V>
where
    V: Animatable,
{
    pub fn new(value: V) -> Self {
        Self { value }
    }
}

impl<V> Default for Constant<V>
where
    V: Animatable + Default,
{
    fn default() -> Self {
        Self::new(V::default())
    }
}
