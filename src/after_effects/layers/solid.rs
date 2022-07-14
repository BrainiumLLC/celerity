use gee::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolidError {}

#[derive(Debug)]
pub struct Solid {
    color: String, // TODO: Convert to SrgbRgba
    size: Size<f64>,
}

impl Solid {
    pub fn from_bodymovin(
        layer: bodymovin::layers::Solid,
        _frame_rate: f64,
        _export_size: &Vec<f64>,
        _canvas_size: &Vec<f64>,
    ) -> Result<Self, SolidError> {
        Ok(Self {
            color: layer.mixin.color,
            size: Size::new(layer.mixin.width, layer.mixin.height),
        })
    }
}
