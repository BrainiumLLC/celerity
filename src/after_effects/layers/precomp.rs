use gee::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PreCompError {}

#[derive(Debug)]
pub struct PreComp {}

impl PreComp {
    pub fn from_bodymovin(
        _layer: bodymovin::layers::PreComp,
        _frame_rate: f64,
        _export_size: &Vec<f64>,
        _canvas_size: &Vec<f64>,
    ) -> Result<Self, PreCompError> {
        Ok(Self {})
    }
}
