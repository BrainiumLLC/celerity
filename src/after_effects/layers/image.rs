use thiserror::Error;

use crate::after_effects::shapes;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("failed to convert `transform`: {0}")]
    TransformInvalid(#[from] shapes::TransformError),
}

#[derive(Debug)]
pub struct Image {
    pub texture_id: String,
    pub transform: shapes::Transform,
}

impl Image {
    pub fn from_bodymovin(
        layer: bodymovin::layers::Image,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        _size_scale: &Vec<f64>,
    ) -> Result<Self, ImageError> {
        Ok(Self {
            transform: shapes::Transform::from_bodymovin_helper(
                layer.transform,
                frame_rate,
                position_scale,
            )?,
            texture_id: layer.mixin.ref_id,
        })
    }
}
