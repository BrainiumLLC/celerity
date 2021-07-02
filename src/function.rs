use crate::{Animatable, Animation};
use std::fmt::{self, Debug};
use time_point::Duration;

/// An animation that just calls a function.
pub struct Function<F, V>
where
    F: Fn(Duration) -> V,
    V: Animatable,
{
    function: F,
}

impl<F, V> Debug for Function<F, V>
where
    F: Fn(Duration) -> V,
    V: Animatable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function").field("function", &"..").finish()
    }
}

impl<F, V> Animation<V> for Function<F, V>
where
    F: Fn(Duration) -> V,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        (self.function)(elapsed)
    }
}

impl<F, V> Function<F, V>
where
    F: Fn(Duration) -> V,
    V: Animatable,
{
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F, V> From<F> for Function<F, V>
where
    F: Fn(Duration) -> V,
    V: Animatable,
{
    fn from(function: F) -> Self {
        Self::new(function)
    }
}
