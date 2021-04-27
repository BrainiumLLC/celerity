use crate::{spline::bezier::cubic_bezier_ease, Animatable, Animation, BoundedAnimation};
use gee::en;
use time_point::Duration;

const SAMPLE_DELTA: f64 = 1e-5;

#[derive(Debug)]
pub struct Interrupt<A, V>
where
    A: Animation<V>,
    V: Animatable,
{
    a: Linear<V>,
    b: A,
    interrupt_t: Duration,
    transition_t: Duration,
}

impl<A, V> Animation<V> for Interrupt<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        // relative change from interrupt_v for each animation
        let a_contribution = self.a.sample(elapsed).zip_map(self.a.value, |a, v| a - v);
        let b_contribution = self.b.sample(elapsed).zip_map(self.a.value, |b, v| b - v);

        // calculate ease
        let transition_percent_elapsed =
            (elapsed.as_secs_f64() / self.transition_t.as_secs_f64()).min(1.0);
        let ease = cubic_bezier_ease(0.333, 0.0, 0.666, 1.0, transition_percent_elapsed);

        // blend a_contribution and b_contribution
        let blended_contributions = a_contribution.zip_map(b_contribution, |a, b| {
            let ac = a * en::cast::<V::Component, _>(1.0 - ease);
            let bc = b;
            ac + bc
        });

        let result = self.a.value.zip_map(blended_contributions, |v, b| v + b);
        result
    }
}

impl<A, V> BoundedAnimation<V> for Interrupt<A, V>
where
    A: BoundedAnimation<V>,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        self.interrupt_t + self.b.duration()
    }
}

impl<A, V> Interrupt<A, V>
where
    A: Animation<V>,
    V: Animatable,
{
    pub fn new(a: A, b: A, interrupt_t: Duration, transition_t: Duration) -> Self {
        let interrupt_v = a.sample(interrupt_t);

        let velocity = a
            .sample(interrupt_t + Duration::from_secs_f64(SAMPLE_DELTA))
            .zip_map(
                a.sample(interrupt_t - Duration::from_secs_f64(SAMPLE_DELTA)),
                |n, p| n - p,
            )
            .map(|a| a * en::cast::<V::Component, _>(0.5 / SAMPLE_DELTA));

        let linear = Linear::new(interrupt_v, velocity);

        Self {
            a: linear,
            b,
            interrupt_t,
            transition_t,
        }
    }

    pub fn with_box(
        a: &Box<dyn Animation<V>>,
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
            )
            .map(|a| a * en::cast::<V::Component, _>(0.5 / SAMPLE_DELTA));

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
pub struct Linear<V> {
    value: V,
    dt_value: V,
}

impl<V> Linear<V>
where
    V: Animatable,
{
    fn new(value: V, dt_value: V) -> Self {
        Self { value, dt_value }
    }
}

impl<V> Animation<V> for Linear<V>
where
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        self.value.zip_map(self.dt_value, |v, dvdt| {
            v + dvdt * en::cast::<V::Component, _>(elapsed.as_secs_f64())
        })
    }
}
