use crate::{
    after_effects::{
        conv::{FromMultiDimensional, FromValue},
        MaybeTrack,
    },
    Animation,
};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RectError {
    #[error("Failed to convert `position`: {0}")]
    PositionInvalid(#[from] <gee::Point<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `size`: {0}")]
    SizeInvalid(#[from] <gee::Size<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `rounded_corners`: {0}")]
    RoundedCornersInvalid(#[from] <f64 as FromValue>::Error),
}

#[derive(Debug)]
pub struct Rect {
    pub direction: f64,
    pub position: MaybeTrack<gee::Point<f64>>,
    pub size: MaybeTrack<gee::Size<f64>>,
    pub rounded_corners: MaybeTrack<f64>,
}

impl Rect {
    pub(crate) fn from_bodymovin(
        rect: bodymovin::shapes::Rect,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Self, RectError> {
        Ok(Self {
            direction: rect.direction,
            position: MaybeTrack::from_multi_dimensional(
                rect.position.scaled(&position_scale),
                frame_rate,
            )?,
            size: MaybeTrack::from_multi_dimensional(rect.size.scaled(&size_scale), frame_rate)?,
            rounded_corners: MaybeTrack::from_property(rect.rounded_corners, frame_rate)
                .map_err(RectError::RoundedCornersInvalid)?,
        })
    }

    pub fn sample_rect(&self, elapsed: Duration) -> gee::Rect<f64> {
        gee::Rect::from_center(self.position.sample(elapsed), self.size.sample(elapsed))
    }

    pub fn sample_rounded_corners(&self, elapsed: Duration) -> f64 {
        self.rounded_corners.sample(elapsed)
    }
}
