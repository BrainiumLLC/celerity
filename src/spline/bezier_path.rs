use crate::{spline::bezier::cubic_bezier, Animatable};

// Describes the two middle control points for a bezier path
// between an interval's spatial endpoints.
//
// These are in absolute coordinates,
// i.e. (from, b1, b2, to) is a bezier.
#[derive(Clone, Debug)]
pub struct BezierPath<V: Animatable> {
    pub b1: V,
    pub b2: V,
}

impl<V: Animatable> BezierPath<V> {
    pub fn new(b1: V, b2: V) -> Self {
        Self { b1, b2 }
    }

    pub fn position(&self, b0: &V, b3: &V, t: f64) -> V {
        cubic_bezier(b0, &self.b1, &self.b2, b3, t)
    }
}
