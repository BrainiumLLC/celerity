use crate::{
    after_effects::{
        conv::{FromMultiDimensional, FromValue},
        MaybeTrack,
    },
    Animation as _,
};
use thiserror::Error;
use time_point::Duration;

#[derive(Debug, Error)]
pub enum SolidError {
    #[error("Failed to convert `color`: {0}")]
    ColorInvalid(#[from] <rainbow::LinRgba as FromMultiDimensional<f64>>::Error),
}

#[derive(Debug, Error)]
pub enum GradientTypeError {
    #[error("Failed to convert `highlight_length`: {0}")]
    HighlightLengthInvalid(#[from] <f64 as FromValue>::Error),
    #[error("Failed to convert `highlight_angle`: {0}")]
    HighlightAngleInvalid(#[from] <gee::Angle<f64> as FromValue<f64>>::Error),
    #[error(
        "Missing `highlight_length` or `highlight_angle` despite being a radial-type gradient"
    )]
    HighlightFieldsMissing,
}

#[derive(Debug, Error)]
pub enum GradientError {
    #[error("Failed to convert `start_point`: {0}")]
    StartPointInvalid(#[source] <gee::Point<f64> as FromMultiDimensional<f64>>::Error),
    #[error("Failed to convert `end_point`: {0}")]
    EndPointInvalid(#[source] <gee::Point<f64> as FromMultiDimensional<f64>>::Error),
    #[error("Failed to classify gradient: {0}")]
    TyInvalid(#[from] GradientTypeError),
}

#[derive(Debug)]
pub enum GradientType {
    Linear,
    // TODO: upstream this nicenes
    Radial {
        highlight_length: MaybeTrack<f64>,
        highlight_angle: MaybeTrack<gee::Angle<f64>, f64>,
    },
}

impl GradientType {
    fn from_bodymovin(
        ty: bodymovin::shapes::GradientType,
        highlight_length: Option<bodymovin::properties::EitherValue>,
        highlight_angle: Option<bodymovin::properties::EitherValue>,
        frame_rate: f64,
    ) -> Result<Self, GradientTypeError> {
        match ty {
            bodymovin::shapes::GradientType::Linear => {
                if highlight_length.is_some() || highlight_angle.is_some() {
                    log::warn!(
                        "gradient specifies a highlight length or angle despite being linear"
                    );
                }
                Ok(Self::Linear)
            }
            bodymovin::shapes::GradientType::Radial => highlight_length
                .zip(highlight_angle)
                .ok_or_else(|| GradientTypeError::HighlightFieldsMissing)
                .and_then(|(highlight_length, highlight_angle)| {
                    Ok(Self::Radial {
                        highlight_length: MaybeTrack::from_value(highlight_length, frame_rate)?,
                        highlight_angle: MaybeTrack::from_value(highlight_angle, frame_rate)?,
                    })
                }),
        }
    }
}

#[derive(Debug)]
pub struct Gradient {
    pub start_point: MaybeTrack<gee::Point<f64>, f64>,
    pub end_point: MaybeTrack<gee::Point<f64>, f64>,
    pub ty: GradientType,
}

#[derive(Debug)]
pub enum Color {
    Solid(MaybeTrack<rainbow::LinRgba, f64>),
    Gradient(Gradient),
}

impl Color {
    pub(crate) fn from_bodymovin_solid(
        color: bodymovin::properties::EitherMultiDimensional,
        frame_rate: f64,
    ) -> Result<Self, SolidError> {
        MaybeTrack::from_multi_dimensional(color, frame_rate)
            .map(Self::Solid)
            .map_err(SolidError::from)
    }

    pub(crate) fn from_bodymovin_gradient(
        start_point: bodymovin::properties::EitherMultiDimensional,
        end_point: bodymovin::properties::EitherMultiDimensional,
        ty: bodymovin::shapes::GradientType,
        highlight_length: Option<bodymovin::properties::EitherValue>,
        highlight_angle: Option<bodymovin::properties::EitherValue>,
        // color: ???,
        frame_rate: f64,
    ) -> Result<Self, GradientError> {
        log::warn!("gradient colors aren't implemented yet");
        Ok(Self::Gradient(Gradient {
            start_point: MaybeTrack::from_multi_dimensional(start_point, frame_rate)
                .map_err(GradientError::StartPointInvalid)?,
            end_point: MaybeTrack::from_multi_dimensional(end_point, frame_rate)
                .map_err(GradientError::EndPointInvalid)?,
            ty: GradientType::from_bodymovin(ty, highlight_length, highlight_angle, frame_rate)?,
        }))
    }

    pub fn sample_color(&self, elapsed: Duration) -> Option<rainbow::LinRgba> {
        if let Self::Solid(color) = self {
            let color = color.sample(elapsed);
            log::info!("sampled color `{:?}`", color);
            Some(color)
        } else {
            None
        }
    }
}
