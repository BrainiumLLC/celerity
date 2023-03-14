use super::bezier::cubic_bezier;
use crate::Animatable;

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

pub struct BezierCurve<V: Animatable> {
    pub b0: V,
    pub b1: V,
    pub b2: V,
    pub b3: V,
}

impl<V: Animatable> BezierCurve<V> {
    pub fn new(b0: V, b1: V, b2: V, b3: V) -> Self {
        Self { b0, b1, b2, b3 }
    }

    pub fn position(&self, t: f64) -> V {
        cubic_bezier(&self.b0, &self.b1, &self.b2, &self.b3, t)
    }
}
