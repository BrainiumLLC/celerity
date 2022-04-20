use crate::after_effects::shapes;
use gee::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextError {
    #[error("failed to convert `transform`: {0}")]
    TransformInvalid(#[from] shapes::TransformError),
}

#[derive(Debug)]
pub struct Text {
    pub transform: shapes::Transform,
    pub text: String,
    pub line_height: f64,
}

impl Text {
    pub fn from_bodymovin(
        layer: bodymovin::layers::Text,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Self, TextError> {
        Ok(Self {
            transform: shapes::Transform::from_bodymovin_helper(
                layer.transform,
                frame_rate,
                position_scale,
            )?,
            text: layer.mixin.text_data.document_data.keyframe_data[0]
                .properties
                .text
                .clone(),
            line_height: layer.mixin.text_data.document_data.keyframe_data[0]
                .properties
                .line_height
                * size_scale[1],
        })
    }
}
