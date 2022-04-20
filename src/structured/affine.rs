use crate::{
    ease::Ease, interval::Interval, retargetable, structured::transform::TransformAnimation,
    Animation,
};
use gee::{Point, Transform};
use std::time::Duration;

// AffineAnimation: Affine Transformations made easy
#[derive(Debug)]
pub struct AffineAnimation {
    pub position: Box<dyn Animation<Point<f32>>>,
    pub transform: TransformAnimation,
}

impl AffineAnimation {
    pub fn from_values(position: Point<f32>, transform: Transform<f32>) -> Self {
        Self {
            position: Box::new(Interval::hold(position, Duration::ZERO)),
            transform: TransformAnimation::hold(transform),
        }
    }

    pub fn sample(&self, elapsed: Duration) -> Transform<f32> {
        let position = self.position.sample(elapsed);
        self.transform
            .sample(elapsed)
            .pre_translate(-position.x, -position.y)
            .post_translate(position.x, position.y)
    }

    pub fn transform(
        &mut self,
        interrupt_t: Duration,
        transition_t: Duration,
        target: Transform<f32>,
        ease: Option<Ease>,
    ) {
        self.transform
            .retarget(interrupt_t, transition_t, target, ease);
    }

    retargetable!(position, Animation, Point<f32>);
}
