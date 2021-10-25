use crate::{interval::Interval, retargetable, spline::bezier_ease::BezierEase, Animation};
use gee::{Angle, DecomposedTransform, Transform, Vector};
use std::time::Duration;

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
        let DecomposedTransform {
            translation: ta,
            rotation: ra,
            skew: ka,
            scale: sa,
        } = from.decompose();
        let DecomposedTransform {
            translation: tb,
            rotation: rb,
            skew: kb,
            scale: sb,
        } = to.decompose();
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
        let identity = DecomposedTransform::identity();
        Self {
            translate: Box::new(Interval::hold(identity.translation, Duration::ZERO)),
            rotate: Box::new(Interval::hold(identity.rotation, Duration::ZERO)),
            scale: Box::new(Interval::hold(identity.scale, Duration::ZERO)),
            skew: Box::new(Interval::hold(identity.skew, Duration::ZERO)),
        }
    }

    pub fn hold(value: Transform<f32>) -> Self {
        let DecomposedTransform {
            translation,
            rotation,
            skew,
            scale,
        } = value.decompose();
        Self {
            translate: Box::new(Interval::hold(translation, Duration::ZERO)),
            rotate: Box::new(Interval::hold(rotation, Duration::ZERO)),
            scale: Box::new(Interval::hold(scale, Duration::ZERO)),
            skew: Box::new(Interval::hold(skew, Duration::ZERO)),
        }
    }

    retargetable!(translate, Animation, Vector<f32>);
    retargetable!(rotate, Animation, Angle<f32>);
    retargetable!(scale, Animation, Vector<f32>);
    retargetable!(skew, Animation, Angle<f32>);

    pub fn retarget(
        &mut self,
        interrupt_t: Duration,
        transition_t: Duration,
        target: Transform<f32>,
        ease: Option<BezierEase>,
    ) {
        let DecomposedTransform {
            translation,
            rotation,
            skew,
            scale,
        } = target.decompose();
        self.translate(interrupt_t, transition_t, translation, ease.clone());
        self.rotate(interrupt_t, transition_t, rotation, ease.clone());
        self.scale(interrupt_t, transition_t, scale, ease.clone());
        self.skew(interrupt_t, transition_t, skew, ease.clone());
    }

    pub fn sample(&self, elapsed: Duration) -> Transform<f32> {
        Transform::from_decomposed(DecomposedTransform {
            translation: self.translate.sample(elapsed),
            rotation: self.rotate.sample(elapsed),
            skew: self.skew.sample(elapsed),
            scale: self.scale.sample(elapsed),
        })
    }
}
