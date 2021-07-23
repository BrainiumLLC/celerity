//! Functions for getting debug info from an animation.

use crate::{Animatable, Animation};
use gee::en::Num as _;
use time_point::Duration;

pub fn path<A, V>(anim: &A, sample_count: usize, sample_duration: Duration) -> Vec<V>
where
    A: Animation<V>,
    V: Animatable,
{
    (0..sample_count + 1)
        .map(|i| anim.sample(sample_duration * (i.to_f64() / sample_count.to_f64())))
        .collect()
}

// Sampling error can occur arround tight curves, showing reduced velocity
pub fn velocity<A, V>(anim: &A, sample_count: usize, sample_duration: Duration) -> Vec<V>
where
    A: Animation<V>,
    V: Animatable,
{
    let sample_delta = sample_duration.as_secs_f64() / sample_count.to_f64();
    path(anim, sample_count + 1, sample_duration)
        .windows(2)
        .map(|window| {
            window[1].zip_map(window[0], |a, b| {
                (a - b) / V::cast_component::<f64>(sample_delta)
            })
        })
        .collect()
}

// Highly sensitive to sampling errors in velocity
pub fn acceleration<A, V>(anim: &A, sample_count: usize, sample_duration: Duration) -> Vec<V>
where
    A: Animation<V>,
    V: Animatable,
{
    velocity(anim, sample_count + 1, sample_duration)
        .windows(2)
        .map(|window| window[1].zip_map(window[0], |a, b| a - b))
        .collect()
}
