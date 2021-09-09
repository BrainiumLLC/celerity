use crate::{
    constant::Constant, interval::Interval, retarget_function, spline::bezier_ease::BezierEase,
    Animation,
};
use gee::{en::Num, Angle, Point, Transform};
use std::f64::consts::TAU;
use time_point::Duration;

// PathAnimation: Animating along a path made simple
// The texture will rotate to follow the path using some RotationStyle,
// but can also be rotated independently of the path's natural curve
#[derive(Debug)]
pub enum RotationStyle {
    // The texture does not rotate
    NoRotation,
    // The texture rotates to align with the path (forward sampling)
    FollowPath,
    // The texture "overcorrects" when rounding curves
    Overcorrect,
}

#[derive(Debug)]
pub struct PathAnimation {
    pub position: Box<dyn Animation<Point<f32>>>,
    pub angle: Box<dyn Animation<Angle<f32>>>,
    style: RotationStyle,
}

impl PathAnimation {
    pub fn from_values(position: Point<f32>, angle: Angle<f32>, style: RotationStyle) -> Self {
        Self {
            position: Box::new(Constant::new(position)),
            angle: Box::new(Constant::new(angle)),
            style,
        }
    }

    pub fn sample_position(&self, elapsed: Duration) -> Point<f32> {
        self.position.sample(elapsed)
    }

    pub fn sample_transform(&self, elapsed: Duration, sample_delta: Duration) -> Transform<f32> {
        Transform::from_rotation_with_fixed_point(
            self.get_angle(elapsed, sample_delta),
            self.sample_position(elapsed),
        )
    }

    pub fn get_angle(&self, elapsed: Duration, sample_delta: Duration) -> Angle {
        match self.style {
            RotationStyle::NoRotation => self.angle.sample(elapsed),
            RotationStyle::FollowPath => {
                let delta =
                    self.position.sample(elapsed + sample_delta) - self.position.sample(elapsed);
                delta.angle() + self.angle.sample(elapsed)
            }
            RotationStyle::Overcorrect => {
                let position = self.position.sample(elapsed);
                let back_angle = (position - self.position.sample(elapsed - sample_delta)).angle();
                let front_angle_difference = (back_angle.radians()
                    - (self.position.sample(elapsed + sample_delta) - position)
                        .angle()
                        .radians())
                .to_f64();

                let shortest_delta = (front_angle_difference.abs() > std::f64::consts::PI)
                    .then(|| {
                        front_angle_difference
                            .is_sign_positive()
                            .then(|| TAU - front_angle_difference)
                            .unwrap_or(-TAU - front_angle_difference)
                    })
                    .unwrap_or(front_angle_difference * -1.0);

                Angle::from_radians(shortest_delta * 1.15).to_f32()
                    + back_angle
                    + self.angle.sample(elapsed)
            }
        }
    }

    retarget_function!(position, Point<f32>);
    retarget_function!(angle, Angle<f32>);
}

impl Animation<Point<f32>> for PathAnimation {
    fn sample(&self, elapsed: Duration) -> Point<f32> {
        self.sample_position(elapsed)
    }
}
