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
pub enum StarTypeError {
    #[error("Failed to convert `inner_radius`: {0}")]
    InnerRadiusInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Failed to convert `inner_roundness`: {0}")]
    InnerRoundnessInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Missing `inner_radius` or `inner_roundness` despite being a star-type star")]
    InnerFieldsMissing,
}

#[derive(Debug, Error)]
pub enum StarError {
    #[error("Failed to convert `position`: {0}")]
    PositionInvalid(#[from] <gee::Point<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `outer_radius`: {0}")]
    OuterRadiusInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Failed to convert `outer_roundness`: {0}")]
    OuterRoundnessInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Failed to convert `rotation`: {0}")]
    RotationInvalid(#[from] <gee::Angle<f64> as FromValue>::Error),
    #[error("Failed to convert `points`: {0}")]
    PointsInvalid(#[from] <u32 as FromValue>::Error),
    #[error("Failed to classify star: {0}")]
    TyInvalid(#[from] StarTypeError),
}

#[derive(Debug)]
pub enum StarType {
    Star {
        // TODO: upstream this enum niceness
        inner_radius: MaybeTrack<f64>,
        inner_roundness: MaybeTrack<f64>,
    },
    Polygon,
}

impl StarType {
    fn from_bodymovin(
        ty: bodymovin::shapes::StarType,
        inner_radius: Option<bodymovin::properties::EitherValue>,
        inner_roundness: Option<bodymovin::properties::EitherValue>,
        frame_rate: f64,
    ) -> Result<Self, StarTypeError> {
        match ty {
            bodymovin::shapes::StarType::Star => inner_radius
                .zip(inner_roundness)
                .ok_or_else(|| StarTypeError::InnerFieldsMissing)
                .and_then(|(inner_radius, inner_roundness)| {
                    Ok(Self::Star {
                        inner_radius: MaybeTrack::from_value(inner_radius, frame_rate)
                            .map_err(StarTypeError::InnerRadiusInvalid)?,
                        inner_roundness: MaybeTrack::from_value(inner_roundness, frame_rate)
                            .map_err(StarTypeError::InnerRoundnessInvalid)?,
                    })
                }),
            bodymovin::shapes::StarType::Polygon => {
                if inner_radius.is_some() || inner_roundness.is_some() {
                    log::warn!("star specifies an inner radius or inner roundness despite being a polygon-type star")
                }
                Ok(Self::Polygon)
            }
        }
    }
}

#[derive(Debug)]
pub struct Star {
    pub direction: f64,
    pub position: MaybeTrack<gee::Point<f64>>,
    pub outer_radius: MaybeTrack<f64>,
    pub outer_roundness: MaybeTrack<f64>,
    pub rotation: MaybeTrack<gee::Angle<f64>>,
    pub points: MaybeTrack<u32>,
    pub ty: StarType,
}

impl Star {
    pub(crate) fn from_bodymovin(
        star: bodymovin::shapes::Star,
        frame_rate: f64,
    ) -> Result<Self, StarError> {
        Ok(Self {
            direction: star.direction,
            position: MaybeTrack::from_multi_dimensional(star.position, frame_rate)?,
            outer_radius: MaybeTrack::from_value(star.outer_radius, frame_rate)
                .map_err(StarError::OuterRadiusInvalid)?,
            outer_roundness: MaybeTrack::from_value(star.outer_roundness, frame_rate)
                .map_err(StarError::OuterRoundnessInvalid)?,
            rotation: MaybeTrack::from_value(star.rotation, frame_rate)?,
            points: MaybeTrack::from_value(star.points, frame_rate)?,
            ty: StarType::from_bodymovin(
                star.ty,
                star.inner_radius,
                star.inner_roundness,
                frame_rate,
            )?,
        })
    }

    pub fn sample_points(&self, elapsed: Duration) -> u32 {
        self.points.sample(elapsed)
    }

    pub fn sample_outer_circle(&self, elapsed: Duration) -> gee::Circle<f64> {
        gee::Circle::new(
            self.position.sample(elapsed),
            self.outer_radius.sample(elapsed),
        )
    }

    pub fn sample_rotation(&self, elapsed: Duration) -> gee::Angle<f64> {
        self.rotation.sample(elapsed)
    }
}
