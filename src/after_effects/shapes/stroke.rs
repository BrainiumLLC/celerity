use crate::{
    after_effects::{
        conv::FromValue,
        shapes::{Color, GradientError, SolidError},
        MaybeTrack,
    },
    Animation as _,
};
pub use bodymovin::helpers::{LineCap, LineJoin};
use thiserror::Error;
use time_point::Duration;

#[derive(Debug, Error)]
pub enum StrokeError {
    #[error("Failed to convert `opacity`: {0}")]
    OpacityInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Failed to convert `width`: {0}")]
    WidthInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Failed to convert `color`: {0}")]
    ColorInvalid(#[from] SolidError),
    #[error("Failed to convert gradient: {0}")]
    GradientInvalid(#[from] GradientError),
}

#[derive(Debug)]
pub struct Stroke {
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: Option<f64>,
    pub opacity: MaybeTrack<f64>,
    pub width: MaybeTrack<f64>,
    pub color: Color,
}

impl Stroke {
    pub(crate) fn from_bodymovin(
        stroke: bodymovin::shapes::Stroke,
        frame_rate: f64,
    ) -> Result<Self, StrokeError> {
        Ok(Self {
            line_cap: stroke.line_cap,
            line_join: stroke.line_join,
            miter_limit: stroke.miter_limit,
            opacity: MaybeTrack::from_value(stroke.opacity, frame_rate)
                .map_err(StrokeError::OpacityInvalid)?,
            width: MaybeTrack::from_value(stroke.width, frame_rate)
                .map_err(StrokeError::WidthInvalid)?,
            color: Color::from_bodymovin_solid(stroke.color, frame_rate)?,
        })
    }

    pub(crate) fn from_bodymovin_with_gradient(
        stroke: bodymovin::shapes::GradientStroke,
        frame_rate: f64,
    ) -> Result<Self, StrokeError> {
        Ok(Self {
            line_cap: stroke.line_cap,
            line_join: stroke.line_join,
            miter_limit: stroke.miter_limit,
            opacity: MaybeTrack::from_value(stroke.opacity, frame_rate)
                .map_err(StrokeError::OpacityInvalid)?,
            // TODO: fix upstream naming inconsistency
            width: MaybeTrack::from_value(stroke.stroke_width, frame_rate)
                .map_err(StrokeError::WidthInvalid)?,
            color: Color::from_bodymovin_gradient(
                stroke.start_point,
                stroke.end_point,
                stroke.ty,
                stroke.highlight_length,
                stroke.highlight_angle,
                frame_rate,
            )?,
        })
    }

    pub fn sample_width(&self, elapsed: Duration) -> f64 {
        self.width.sample(elapsed)
    }
}
