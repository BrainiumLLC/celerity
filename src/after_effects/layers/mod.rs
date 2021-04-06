mod shape;

pub use self::shape::Shape;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Layer {
    Shape(Shape),
    Unimplemented,
}

impl Layer {
    pub(crate) fn from_bodymovin(layer: bodymovin::layers::Layer, frame_rate: f64) -> Self {
        match layer {
            bodymovin::layers::Layer::Shape(layer) => {
                Self::Shape(Shape::from_bodymovin(layer, frame_rate))
            }
            _ => Self::Unimplemented,
        }
    }
}
