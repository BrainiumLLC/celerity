use crate::after_effects::shapes;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShapeError {
    #[error("failed to convert `transform`: {0}")]
    TransformInvalid(#[from] shapes::TransformError),
    #[error("failed to convert `shapes`: {0}")]
    ShapeInvalid(#[from] shapes::Error),
}

#[derive(Debug)]
pub struct Shape {
    pub transform: shapes::Transform,
    pub shapes: Vec<shapes::Shape>,
}

impl Shape {
    pub fn from_bodymovin(
        layer: bodymovin::layers::Shape,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Self, ShapeError> {
        Ok(Self {
            transform: shapes::Transform::from_bodymovin_helper(
                layer.transform,
                frame_rate,
                position_scale,
            )?,
            shapes: layer
                .mixin
                .shapes
                .into_iter()
                // TODO: what to do with the options here?
                .flat_map(|shape| {
                    shapes::Shape::from_bodymovin(shape, frame_rate, position_scale, size_scale)
                        .transpose()
                })
                .collect::<Result<_, _>>()?,
        })
    }
}
