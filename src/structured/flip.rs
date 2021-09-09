use crate::{
    constant::Constant, interval::Interval, retargetable, spline::bezier_ease::BezierEase,
    Animation,
};

use gee::{Angle, Point, Transform, Transform3d};

use time_point::Duration;

const MAX_DISTORT: f32 = 0.0005;

// FlipAnimation: Examining correct use of perspective
#[derive(Debug)]
pub struct FlipAnimation {
    pub position: Box<dyn Animation<Point<f32>>>,
    // This is the angle describing the axis about which the face of the card will rotate
    pub angle: Box<dyn Animation<Angle<f32>>>, // can this be animated? Work it out!
    // This is the angle of rotation about the axis described by "angle"
    pub flip: Box<dyn Animation<Angle<f32>>>, // amount of flip which has occurred
    pub scale: Box<dyn Animation<f32>>,       // Scaling
}

impl FlipAnimation {
    pub fn from_values(
        position: Point<f32>,
        angle: Angle<f32>,
        flip: Angle<f32>,
        scale: f32,
    ) -> Self {
        Self {
            position: Box::new(Constant::new(position)),
            angle: Box::new(Constant::new(angle)),
            flip: Box::new(Constant::new(flip)),
            scale: Box::new(Constant::new(scale)),
        }
    }

    pub fn from_animations(
        position: Box<dyn Animation<Point<f32>>>,
        angle: Box<dyn Animation<Angle<f32>>>,
        flip: Box<dyn Animation<Angle<f32>>>,
        scale: Box<dyn Animation<f32>>,
    ) -> Self {
        Self {
            position,
            angle,
            flip,
            scale,
        }
    }

    pub fn sample(&self, elapsed: Duration) -> Transform3d<f32> {
        let position = self.position.sample(elapsed);
        let angle = self.angle.sample(elapsed);
        let scale = self.scale.sample(elapsed);
        let scale_percent = self.get_scale_percent(elapsed);
        let faceup = self.is_faceup(elapsed);
        let distortion = Self::get_distortion(scale_percent, faceup, self.is_returning(elapsed));
        // When facedown, the resting angle will be offset by 2x the angle of rotation
        let offset = (Angle::PI() - (angle * 2.0)) * (1.0 - faceup as i32 as f32);

        // Translate to origin, rotate & scale
        let pre_transform: Transform3d<f32> = Transform::from_translation(-position.x, -position.y)
            .post_rotate(angle + offset)
            .post_scale(scale, scale * (1.0 - scale_percent))
            .into();

        // Unrotate, translate back to self.position
        let post_transform: Transform3d<f32> = Transform::from_rotation(-angle)
            .post_translate(position.x, position.y)
            .into();

        pre_transform
            // Distort (but only on y, due to rotation)
            .post_mul(Transform3d {
                m24: distortion,
                ..Transform3d::identity()
            })
            .post_mul(post_transform)
    }

    pub fn is_faceup(&self, elapsed: Duration) -> bool {
        let angle = self.flip.sample(elapsed).normalize();
        angle > -Angle::FRAC_PI_2() && angle < Angle::FRAC_PI_2()
    }

    pub fn is_returning(&self, elapsed: Duration) -> bool {
        self.flip.sample(elapsed).normalize().radians() < 0.0
    }

    fn get_distortion(flip_percent: f32, faceup: bool, returning: bool) -> f32 {
        flip_percent
            * if faceup ^ returning {
                -MAX_DISTORT
            } else {
                MAX_DISTORT
            }
    }

    pub fn get_scale_percent(&self, elapsed: Duration) -> f32 {
        let flip = self.flip.sample(elapsed).normalize().radians();

        if self.is_faceup(elapsed) {
            // in range -pi/2 to +pi/2
            (flip / std::f32::consts::FRAC_PI_2).abs()
        } else {
            // in range -pi - -pi/2 & pi/2+
            1.0 - ((flip.abs() - std::f32::consts::FRAC_PI_2) / std::f32::consts::FRAC_PI_2)
        }
    }

    retargetable!(position, Animation, Point<f32>);
    retargetable!(angle, Animation, Angle<f32>);
    retargetable!(flip, Animation, Angle<f32>);
    retargetable!(scale, Animation, f32);
}
