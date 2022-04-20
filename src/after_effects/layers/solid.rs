use gee::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolidError {}

#[derive(Debug)]
pub struct Solid {}

impl Solid {
    pub fn from_bodymovin(
        _layer: bodymovin::layers::Solid,
        _frame_rate: f64,
        _export_size: &Vec<f64>,
        _canvas_size: &Vec<f64>,
    ) -> Result<Self, SolidError> {
        Ok(Self {})
    }
}
