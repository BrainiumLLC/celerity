mod shape;

pub use self::shape::*;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LayerError {
    #[error("Failed to convert shape layer: {0}")]
    ShapeInvalid(#[from] ShapeError),
    #[error("Unimplemented layer type: {0:?}")]
    Unimplemented(bodymovin::layers::Layer),
}

#[derive(Debug)]
pub enum Layer {
    Shape(Shape),
}

impl Layer {
    pub(crate) fn from_bodymovin(
        layer: bodymovin::layers::Layer,
        frame_rate: f64,
    ) -> Result<Self, LayerError> {
        match layer {
            bodymovin::layers::Layer::Shape(layer) => Shape::from_bodymovin(layer, frame_rate)
                .map(Self::Shape)
                .map_err(LayerError::from),
            _ => Err(LayerError::Unimplemented(layer)),
        }
    }
}
