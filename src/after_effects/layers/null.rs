use gee::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NullError {}

#[derive(Debug)]
pub struct Null {}

impl Null {
    pub fn from_bodymovin(
        _layer: bodymovin::layers::Null,
        _frame_rate: f64,
        _export_size: &Vec<f64>,
        _canvas_size: &Vec<f64>,
    ) -> Result<Self, NullError> {
        Ok(Self {})
    }
}
