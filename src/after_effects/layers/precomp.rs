use thiserror::Error;

use crate::after_effects::shapes;

#[derive(Debug, Error)]
pub enum PreCompError {
    #[error("failed to convert `transform`: {0}")]
    TransformInvalid(#[from] shapes::TransformError),
}

#[derive(Debug)]
pub struct PreComp {
    pub id: String,
    // TODO: Layers from PreComp assets
    // layers: Vec<AnyLayer>,
    pub transform: shapes::Transform,
}

impl PreComp {
    pub fn from_bodymovin(
        layer: bodymovin::layers::PreComp,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        _canvas_size: &Vec<f64>,
    ) -> Result<Self, PreCompError> {
        Ok(Self {
            id: layer.mixin.ref_id,
            // layers: layer.layers,
            transform: shapes::Transform::from_bodymovin_helper(
                layer.transform,
                frame_rate,
                position_scale,
            )?,
        })
    }
}
