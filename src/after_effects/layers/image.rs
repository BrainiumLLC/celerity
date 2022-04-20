use gee::Size;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {}

#[derive(Debug)]
pub struct Image {}

impl Image {
    pub fn from_bodymovin(
        _layer: bodymovin::layers::Image,
        _frame_rate: f64,
        _export_size: &Vec<f64>,
        _canvas_size: &Vec<f64>,
    ) -> Result<Self, ImageError> {
        Ok(Self {})
    }
}
