use crate::{spline::bezier::cubic_bezier_ease, Animatable, Animation, BoundedAnimation};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

const SAMPLE_DELTA: f64 = 1e-5;

#[derive(Debug)]
pub struct Interrupt<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    a: Linear<V, C>,
    b: A,
    interrupt_t: Duration,
    transition_t: Duration,
}

impl<A, V, C> Animation<V, C> for Interrupt<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn sample(&self, elapsed: Duration) -> V {
        // relative change from interrupt_v for each animation
        let a_contribution = self.a.sample(elapsed).zip_map(self.a.value, |a, v| a - v);
        let b_contribution = self.b.sample(elapsed).zip_map(self.a.value, |b, v| b - v);

        // calculate ease
        let transition_percent_elapsed =
            (elapsed.as_secs_f64() / self.transition_t.as_secs_f64()).min(1.0);
        let ease = cubic_bezier_ease(0.166, 0.0, 0.834, 1.0, transition_percent_elapsed);

        // blend a_contribution and b_contribution
        let blended_contributions = a_contribution.zip_map(b_contribution, |a, b| {
            let ac = a * en::cast::<C, _>(1.0 - ease);
            let bc = b * en::cast::<C, _>(ease);
            ac + bc
        });

        let result = self.a.value.zip_map(blended_contributions, |v, b| v + b);
        result
    }
}

impl<A, V, C> BoundedAnimation<V, C> for Interrupt<A, V, C>
where
    A: BoundedAnimation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    fn duration(&self) -> Duration {
        self.interrupt_t + self.b.duration()
    }
}

impl<A, V, C> Interrupt<A, V, C>
where
    A: Animation<V, C>,
    V: Animatable<C>,
    C: en::Num,
{
    pub fn new(a: A, b: A, interrupt_t: Duration, transition_t: Duration) -> Self {
        let interrupt_v = a.sample(interrupt_t);

        let velocity = a
            .sample(interrupt_t + Duration::from_secs_f64(SAMPLE_DELTA))
            .zip_map(
                a.sample(interrupt_t - Duration::from_secs_f64(SAMPLE_DELTA)),
                |n, p| n - p,
            ).map(|a| a * en::cast::<C, _>(0.5 / SAMPLE_DELTA));

        let linear = Linear::new(interrupt_v, velocity);

        Self {
            a: linear,
            b,
            interrupt_t,
            transition_t,
        }
    }

    pub fn with_box(
        a: &Box<dyn Animation<V, C>>,
        b: A,
        interrupt_t: Duration,
        transition_t: Duration,
    ) -> Self {
        let interrupt_v = a.sample(interrupt_t);

        let velocity = a
            .sample(interrupt_t + Duration::from_secs_f64(SAMPLE_DELTA))
            .zip_map(
                a.sample(interrupt_t - Duration::from_secs_f64(SAMPLE_DELTA)),
                |n, p| n - p,
            ).map(|a| a * en::cast::<C, _>(0.5 / SAMPLE_DELTA));

        let linear = Linear::new(interrupt_v, velocity);

        Self {
            a: linear,
            b,
            interrupt_t,
            transition_t,
        }
    }
}

// Sampleable Linear animation from a point w/ a vector
#[derive(Debug)]
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
        Self {
            value,
            dt_value,
            _marker: PhantomData,
        }
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
