use crate::{
    interval::{Frame, Interval},
    spline::{bezier::cubic_bezier_ease, bezier_ease::BezierEase},
    Animatable, Animation, BoundedAnimation,
};
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
        let interrupt_v = self.a.sample(self.interrupt_t);

        let a_contribution = self.a.sample(elapsed).zip_map(interrupt_v, |a, v| a - v);
        let b_contribution = self.b.sample(elapsed).zip_map(interrupt_v, |b, v| b - v);

        // calculate ease
        let transition_percent_elapsed = ((elapsed.as_secs_f64() - self.interrupt_t.as_secs_f64())
            / self.transition_t.as_secs_f64())
        .min(1.0);
        let ease = BezierEase::ease_in_out().ease(transition_percent_elapsed);

        // blend a_contribution and b_contribution
        let blended_contributions = a_contribution.zip_map(b_contribution, |a, b| {
            let ac = a * en::cast::<V::Component, _>(1.0 - ease);
            let bc = b;
            ac + bc
        });

        interrupt_v.zip_map(blended_contributions, |v, b| v + b)
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
    pub fn boxed(
        a: &Box<dyn Animation<V>>,
        b: A,
        interrupt_t: Duration,
        transition_t: Duration,
    ) -> Box<Self> {
        let interrupt_v = a.sample(interrupt_t);

        let velocity = a
            .sample(interrupt_t + Duration::from_secs_f64(SAMPLE_DELTA))
            .zip_map(
                a.sample(interrupt_t - Duration::from_secs_f64(SAMPLE_DELTA)),
                |n, p| n - p,
            )
            .map(|a| a * en::cast::<V::Component, _>(0.5 / SAMPLE_DELTA));

        // linear should produce interrupt_v when sampled at interrupt_t
        let start_v = interrupt_v.zip_map(
            velocity.map(|v| v * en::cast::<V::Component, _>(interrupt_t.as_secs_f64())),
            |i, v| i - v,
        );
        let linear = Linear::new(start_v, velocity);

        Box::new(Self {
            a: linear,
            b,
            interrupt_t,
            transition_t,
        })
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
