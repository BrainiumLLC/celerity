use crate::{
    interval::Interval, spline::bezier_ease::BezierEase, Animatable, Animation, BoundedAnimation,
};
use gee::en;
use time_point::Duration;

const SAMPLE_DELTA: f64 = 1e-5;

#[derive(Debug)]
pub struct Interrupt<A, B, V>
where
    A: Animation<V>,
    B: Animation<V>,
    V: Animatable,
{
    a: Option<A>,
    a_interrupt: Linear<V>,
    b: B,
    interrupt_t: Duration,
    transition_t: Duration,
    pre_multiplied: bool,
}

impl<A, B, V> Animation<V> for Interrupt<A, B, V>
where
    A: Animation<V>,
    B: Animation<V>,
    V: Animatable,
{
    fn sample(&self, elapsed: Duration) -> V {
        if elapsed >= self.interrupt_t {
            let elapsed = elapsed - self.interrupt_t;

            // relative change from interrupt.value for each animation
            let a_contribution = self
                .a_interrupt
                .sample(elapsed)
                .zip_map(self.a_interrupt.value, |a, v| a - v);
            let b_contribution = self
                .b
                .sample(elapsed)
                .zip_map(self.a_interrupt.value, |b, v| b - v);

            // calculate ease
            let transition_percent_elapsed =
                ((elapsed.as_secs_f64()) / self.transition_t.as_secs_f64()).min(1.0);
            let ease = BezierEase::ease_in_out().ease(transition_percent_elapsed);

            // blend a_contribution and b_contribution
            let blended_contributions = a_contribution.zip_map(b_contribution, |a, b| {
                let ac = a * en::cast::<V::Component, _>(1.0 - ease);
                let bc = if self.pre_multiplied {
                    b
                } else {
                    b * en::cast::<V::Component, _>(ease)
                };
                ac + bc
            });

            self.a_interrupt
                .value
                .zip_map(blended_contributions, |v, b| v + b)
        } else {
            if let Some(animation) = &self.a {
                animation.sample(elapsed)
            } else {
                self.a_interrupt.sample(elapsed)
            }
        }
    }
}

impl<A, B, V> BoundedAnimation<V> for Interrupt<A, B, V>
where
    A: Animation<V>,
    B: BoundedAnimation<V>,
    V: Animatable,
{
    fn duration(&self) -> Duration {
        self.interrupt_t + self.b.duration()
    }
}

impl<A, B, V> Interrupt<A, B, V>
where
    A: Animation<V>,
    B: Animation<V>,
    V: Animatable,
{
    pub fn new(a: A, b: B, interrupt_t: Duration, transition_t: Duration) -> Self {
        Self {
            a: None,
            ..Self::reversible(a, b, interrupt_t, transition_t)
        }
    }

    pub fn reversible(a: A, b: B, interrupt_t: Duration, transition_t: Duration) -> Self {
        let interrupt_v = a.sample(interrupt_t);

        let velocity = a
            .sample(interrupt_t + Duration::from_secs_f64(SAMPLE_DELTA))
            .zip_map(
                a.sample(interrupt_t - Duration::from_secs_f64(SAMPLE_DELTA)),
                |n, p| n - p,
            )
            .map(|a| a * en::cast::<V::Component, _>(0.5 / SAMPLE_DELTA));

        let linear = Linear::new(interrupt_v, velocity);

        let pre_multiplied = b.sample(Duration::zero()).distance_to(interrupt_v) == 0.0;

        Self {
            a: Some(a),
            a_interrupt: linear,
            b,
            interrupt_t,
            transition_t,
            pre_multiplied,
        }
    }
}

// Linear animation from a point w/ a vector
#[derive(Debug)]
pub struct Linear<V> {
    pub value: V,
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
