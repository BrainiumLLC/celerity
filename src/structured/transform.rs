use crate::interval::Interval;
use crate::spline::bezier_ease::BezierEase;
use crate::Animation;
use gee::{Angle, Transform, Vector};

use time_point::Duration;

use crate::retarget_function;

// TransformAnimation: Working with Affine transformations
#[derive(Debug)]
pub struct TransformAnimation {
    pub translate: Box<dyn Animation<Vector<f32>>>,
    pub rotate: Box<dyn Animation<Angle<f32>>>,
    pub scale: Box<dyn Animation<Vector<f32>>>,
    pub skew: Box<dyn Animation<Angle<f32>>>,
}

impl TransformAnimation {
    pub fn new(
        start: Duration,
        duration: Duration,
        from: Transform<f32>,
        to: Transform<f32>,
        ease: Option<BezierEase>,
    ) -> Self {
        let (ta, ra, ka, sa) = from.decompose();
        let (tb, rb, kb, sb) = to.decompose();

        Self {
            translate: Box::new(Interval::new(
                start,
                duration,
                ta,
                tb,
                ease.clone(),
                None,
                None,
            )),
            rotate: Box::new(Interval::new(
                start,
                duration,
                ra,
                rb,
                ease.clone(),
                None,
                None,
            )),
            scale: Box::new(Interval::new(
                start,
                duration,
                sa,
                sb,
                ease.clone(),
                None,
                None,
            )),
            skew: Box::new(Interval::new(
                start,
                duration,
                ka,
                kb,
                ease.clone(),
                None,
                None,
            )),
        }
    }

    pub fn identity() -> Self {
        Self {
            translate: Box::new(Interval::hold(Vector::zero(), Duration::zero())),
            rotate: Box::new(Interval::hold(Angle::ZERO(), Duration::zero())),
            scale: Box::new(Interval::hold(Vector::new(1.0, 1.0), Duration::zero())),
            skew: Box::new(Interval::hold(Angle::ZERO(), Duration::zero())),
        }
    }

    pub fn hold(value: Transform<f32>) -> Self {
        let (translation, rotation, skew, scale) = value.decompose();

        Self {
            translate: Box::new(Interval::hold(translation, Duration::zero())),
            rotate: Box::new(Interval::hold(rotation, Duration::zero())),
            scale: Box::new(Interval::hold(scale, Duration::zero())),
            skew: Box::new(Interval::hold(skew, Duration::zero())),
        }
    }

    retarget_function!(translate, Vector<f32>);
    retarget_function!(rotate, Angle<f32>);
    retarget_function!(scale, Vector<f32>);
    retarget_function!(skew, Angle<f32>);

    pub fn retarget(
        &mut self,
        interrupt_t: Duration,
        transition_t: Duration,
        target: Transform<f32>,
        ease: Option<BezierEase>,
    ) {
        let (translation, rotation, skew, scale) = target.decompose();

        self.translate(interrupt_t, transition_t, translation, ease.clone());
        self.rotate(interrupt_t, transition_t, rotation, ease.clone());
        self.scale(interrupt_t, transition_t, scale, ease.clone());
        self.skew(interrupt_t, transition_t, skew, ease.clone());
    }

    pub fn sample(&self, elapsed: Duration) -> Transform<f32> {
        Transform::<f32>::from_decomposition(
            self.translate.sample(elapsed),
            self.rotate.sample(elapsed),
            self.skew.sample(elapsed),
            self.scale.sample(elapsed),
        )
        .into()
    }

    pub fn sample_transform(&self, elapsed: Duration) -> Transform<f32> {
        Transform::<f32>::from_decomposition(
            self.translate.sample(elapsed),
            self.rotate.sample(elapsed),
            self.skew.sample(elapsed),
            self.scale.sample(elapsed),
        )
    }
}
