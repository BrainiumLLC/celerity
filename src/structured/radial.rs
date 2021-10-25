use crate::{
    constant::Constant, interval::Interval, retargetable, spline::bezier_ease::BezierEase,
    Animation, BoundedAnimation,
};
use gee::{Angle, Circle, Point};
use replace_with::replace_with_or_abort;
use std::time::Duration;

// Radial: A fun test for Celerity
// An animation which travels around a circle, varying its angle and distance from an origin point
#[derive(Debug)]
pub struct Radial {
    pub origin: Box<dyn Animation<Point<f32>>>,
    pub angle: Box<dyn Animation<Angle<f32>>>,
    pub distance: Box<dyn Animation<f32>>,
}

impl Radial {
    pub fn new(origin: Point<f32>, angle: Angle<f32>, distance: f32) -> Self {
        Self {
            origin: Box::new(Constant::new(origin)),
            angle: Box::new(Constant::new(angle)),
            distance: Box::new(Constant::new(distance)),
        }
    }

    pub fn clockwise(&mut self, interrupt_t: Duration, speed: Duration) {
        let interrupt_v = self.angle.sample(interrupt_t);

        replace_with_or_abort(&mut self.angle, |angle| {
            Box::new(
                angle
                    .interrupt(
                        Interval::from_values(
                            speed,
                            interrupt_v,
                            interrupt_v - (Angle::PI() * 2.0),
                            None,
                        ),
                        interrupt_t,
                        speed,
                    )
                    .cycle(),
            )
        });
    }

    pub fn anticlockwise(&mut self, interrupt_t: Duration, speed: Duration) {
        let interrupt_v = self.angle.sample(interrupt_t);

        replace_with_or_abort(&mut self.angle, |angle| {
            Box::new(
                angle
                    .interrupt(
                        Interval::from_values(
                            speed,
                            interrupt_v,
                            interrupt_v + (Angle::PI() * 2.0),
                            None,
                        ),
                        interrupt_t,
                        speed,
                    )
                    .cycle(),
            )
        });
    }

    pub fn to_and_from(&mut self, to: f32, from: f32, interrupt_t: Duration, speed: Duration) {
        let interrupt_v = self.distance.sample(interrupt_t);

        replace_with_or_abort(&mut self.distance, |distance| {
            Box::new(
                distance.interrupt(
                    Interval::from_values(speed / 2, interrupt_v, to, Some(BezierEase::ease_in()))
                        .chain(
                            Interval::from_values(speed / 2, to, from, None)
                                .chain(Interval::from_values(speed / 2, from, to, None))
                                .cycle(),
                        ),
                    interrupt_t,
                    speed,
                ),
            )
        });
    }

    retargetable!(distance, Animation, f32);
    retargetable!(origin, Animation, Point<f32>);
    retargetable!(angle, Animation, Angle<f32>);
}

impl Animation<Point<f32>> for Radial {
    fn sample(&self, elapsed: Duration) -> Point<f32> {
        Circle::circle_points(
            &Circle::new(self.origin.sample(elapsed), self.distance.sample(elapsed)),
            1,
            self.angle.sample(elapsed),
        )
        .last()
        .unwrap()
    }
}
