use crate::after_effects::{
    conv::FromValue,
    shapes::{Color, GradientError, SolidError},
    MaybeTrack,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FillError {
    #[error("Failed to convert `opacity`: {0}")]
    OpacityInvalid(#[from] <f64 as FromValue>::Error),
    #[error("Failed to convert `color`: {0}")]
    ColorInvalid(#[from] SolidError),
    #[error("Failed to convert gradient: {0}")]
    GradientInvalid(#[from] GradientError),
}

#[derive(Debug)]
pub struct Fill {
    pub opacity: MaybeTrack<f64>,
    pub color: Color,
}

impl Fill {
    pub(crate) fn from_bodymovin(
        fill: bodymovin::shapes::Fill,
        frame_rate: f64,
    ) -> Result<Self, FillError> {
        Ok(Self {
            opacity: MaybeTrack::from_property(fill.opacity, frame_rate)?,
            color: Color::from_bodymovin_solid(fill.color, frame_rate)?,
        })
    }

    pub(crate) fn from_bodymovin_with_gradient(
        fill: bodymovin::shapes::GradientFill,
        frame_rate: f64,
    ) -> Result<Self, FillError> {
        Ok(Self {
            opacity: MaybeTrack::from_property(fill.opacity, frame_rate)?,
            color: Color::from_bodymovin_gradient(
                fill.start_point,
                fill.end_point,
                fill.ty,
                Some(fill.highlight_length.unwrap_or_default().value),
                Some(fill.highlight_angle.unwrap_or_default().value),
                frame_rate,
            )?,
        })
    }
}
