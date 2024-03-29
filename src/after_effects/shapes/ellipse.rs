use crate::after_effects::{conv::FromMultiDimensional, MaybeTrack};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EllipseError {
    #[error("Failed to convert `position`: {0}")]
    PositionInvalid(#[from] <gee::Point<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `size`: {0}")]
    SizeInvalid(#[from] <gee::Size<f64> as FromMultiDimensional>::Error),
}

#[derive(Debug)]
pub struct Ellipse {
    pub direction: f64,
    pub position: MaybeTrack<gee::Point<f64>>,
    pub size: MaybeTrack<gee::Size<f64>>,
}

impl Ellipse {
    pub(crate) fn from_bodymovin(
        ellipse: bodymovin::shapes::Ellipse,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Self, EllipseError> {
        Ok(Self {
            direction: ellipse.direction,
            position: MaybeTrack::from_multi_dimensional(
                ellipse.position.scaled(position_scale),
                frame_rate,
            )?,
            size: MaybeTrack::from_multi_dimensional(ellipse.size.scaled(size_scale), frame_rate)?,
        })
    }
}
