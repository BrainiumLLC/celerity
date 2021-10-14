use crate::{
    interval::Interval, retargetable, spline::bezier_ease::BezierEase, Animation, BoundedAnimation,
};
use replace_with::replace_with_or_abort;
use std::time::Duration;

const TRANSITION_TIME: f64 = 0.5;

// Clock: A more convenient animation clock
#[derive(Debug)]
pub struct Clock {
    pub now: Duration,
    pub total_elapsed: Duration,
    pub rate_of_travel: Box<dyn Animation<f64>>,
    interrupt_t: Duration,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            now: Duration::ZERO,
            total_elapsed: Duration::ZERO,
            rate_of_travel: Box::new(Interval::hold(1.0, Duration::ZERO)),
            interrupt_t: Duration::ZERO,
        }
    }
}

impl Clock {
    pub fn new(now: Duration, rate_of_travel: f64) -> Self {
        Self {
            now,
            total_elapsed: Duration::ZERO,
            rate_of_travel: Box::new(Interval::hold(rate_of_travel, Duration::ZERO)),
            interrupt_t: Duration::ZERO,
        }
    }

    pub fn time_passed(&mut self, elapsed: Duration) {
        self.now += elapsed.mul_f64(self.rate_of_travel.sample(self.total_elapsed));
        self.total_elapsed += elapsed;
    }

    pub fn normal_speed(&mut self) {
        self.rate_of_travel(
            self.total_elapsed,
            Duration::from_secs_f64(1.0),
            1.0,
            Some(BezierEase::ease_in_out()),
        );
    }

    pub fn slow_speed(&mut self) {
        self.rate_of_travel(
            self.total_elapsed,
            Duration::from_secs_f64(1.0),
            0.1,
            Some(BezierEase::ease_in_out()),
        );
    }

    pub fn bullet_time(&mut self, duration: Duration) {
        self.temporary_speed_change(0.1, duration);
    }

    pub fn fast_forward(&mut self, duration: Duration) {
        self.temporary_speed_change(2.0, duration);
    }

    pub fn rewind(&mut self, duration: Duration) {
        self.temporary_speed_change(-1.0, duration);
    }

    fn temporary_speed_change(&mut self, target: f64, duration: Duration) {
        let interrupt_t = self.total_elapsed;
        let transition_t = Duration::from_secs_f64(
            TRANSITION_TIME
                .min(duration.as_secs_f64() / 2.0)
                .min(self.total_elapsed.as_secs_f64() - self.interrupt_t.as_secs_f64()),
        );
        let from = self.rate_of_travel.sample(self.total_elapsed);

        replace_with_or_abort(&mut self.rate_of_travel, |rate_of_travel| {
            Box::new(
                rate_of_travel.interrupt(
                    Interval::from_values(
                        transition_t,
                        from,
                        target,
                        Some(BezierEase::ease_in_out()),
                    )
                    .chain(Interval::hold(target, duration - (transition_t * 2)))
                    .chain(Interval::from_values(
                        transition_t,
                        target,
                        1.0,
                        Some(BezierEase::ease_in_out()),
                    )),
                    interrupt_t,
                    transition_t,
                ),
            )
        });
    }

    retargetable!(rate_of_travel, Animation, f64);
}
