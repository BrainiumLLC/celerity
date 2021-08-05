use crate::{
    after_effects::{
        conv::{FromMultiDimensional, FromValue},
        MaybeTrack,
    },
    Animation,
};
use thiserror::Error;
use time_point::Duration;

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Failed to convert `anchor_point`: {0}")]
    AnchorPointInvalid(#[source] <gee::Vector<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `position`: {0}")]
    PositionInvalid(#[source] <gee::Vector<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `scale`: {0}")]
    ScaleInvalid(#[source] <gee::Vector<f64> as FromMultiDimensional>::Error),
    #[error("Failed to convert `rotation`: {0}")]
    RotationInvalid(#[source] <gee::Angle<f64> as FromValue>::Error),
    #[error("Failed to convert `opacity`: {0}")]
    OpacityInvalid(#[source] <f64 as FromValue>::Error),
    #[error("Failed to convert `skew`: {0}")]
    SkewInvalid(#[source] <gee::Angle<f64> as FromValue>::Error),
    #[error("Failed to convert `skew_axis`: {0}")]
    SkewAxisInvalid(#[source] <f64 as FromValue>::Error),
}

#[derive(Debug)]
pub struct Transform {
    anchor_point: MaybeTrack<gee::Vector<f64>>,
    position: MaybeTrack<gee::Vector<f64>>,
    scale: MaybeTrack<gee::Vector<f64>>,
    rotation: MaybeTrack<gee::Angle<f64>>,
    opacity: MaybeTrack<f64>,
    skew: MaybeTrack<gee::Angle<f64>>,
    skew_axis: MaybeTrack<f64>,
}

impl Transform {
    pub(crate) fn from_bodymovin_helper(
        transform: bodymovin::helpers::Transform,
        frame_rate: f64,
    ) -> Result<Self, TransformError> {
        // TODO: we're not handling px/py/pz
        Ok(Self {
            anchor_point: MaybeTrack::from_multi_dimensional(transform.anchor_point, frame_rate)
                .map_err(TransformError::AnchorPointInvalid)?,
            position: MaybeTrack::from_multi_dimensional(transform.position, frame_rate)
                .map_err(TransformError::PositionInvalid)?,
            scale: MaybeTrack::from_multi_dimensional(transform.scale, frame_rate)
                .map_err(TransformError::ScaleInvalid)?,
            rotation: MaybeTrack::from_value(transform.rotation, frame_rate)
                .map_err(TransformError::RotationInvalid)?,
            opacity: MaybeTrack::from_value(transform.opacity, frame_rate)
                .map_err(TransformError::OpacityInvalid)?,
            skew: MaybeTrack::from_value(transform.skew, frame_rate)
                .map_err(TransformError::SkewInvalid)?,
            skew_axis: MaybeTrack::from_value(transform.skew_axis, frame_rate)
                .map_err(TransformError::SkewAxisInvalid)?,
        })
    }

    pub(crate) fn from_bodymovin_shape(
        transform: bodymovin::shapes::Transform,
        frame_rate: f64,
    ) -> Result<Self, TransformError> {
        Ok(Self {
            anchor_point: MaybeTrack::from_multi_dimensional(transform.anchor_point, frame_rate)
                .map_err(TransformError::AnchorPointInvalid)?,
            position: MaybeTrack::from_multi_dimensional(transform.position, frame_rate)
                .map_err(TransformError::PositionInvalid)?,
            scale: MaybeTrack::from_multi_dimensional(transform.scale, frame_rate)
                .map_err(TransformError::ScaleInvalid)?,
            rotation: MaybeTrack::from_value(transform.rotation, frame_rate)
                .map_err(TransformError::RotationInvalid)?,
            opacity: MaybeTrack::from_value(transform.opacity, frame_rate)
                .map_err(TransformError::OpacityInvalid)?,
            skew: MaybeTrack::from_value(transform.skew, frame_rate)
                .map_err(TransformError::SkewInvalid)?,
            skew_axis: MaybeTrack::from_value(transform.skew_axis, frame_rate)
                .map_err(TransformError::SkewAxisInvalid)?,
        })
    }

    pub fn sample_transform(&self, elapsed: Duration) -> gee::Transform<f64> {
        let anchor_point = self.anchor_point.sample(elapsed);
        let translation = self.position.sample(elapsed);
        let rotation = self.rotation.sample(elapsed);
        let skew = self.skew.sample(elapsed);
        // scale is a percentage!
        let scale = self.scale.sample(elapsed) / gee::Vector::uniform(100.0);
        let transform = gee::DecomposedTransform {
            translation,
            rotation,
            skew,
            scale,
        };
        log::info!("sampled transform components: `{:#?}`", transform,);
        // TODO: what to do with skew_axis?
        gee::Transform::from_decomposed(transform).pre_translate_vector(-anchor_point)
    }

    pub fn sample_opacity(&self, elapsed: Duration) -> f64 {
        let opacity = self.opacity.sample(elapsed) / 100.0;
        log::info!("sampled opacity {:?}", opacity);
        opacity
    }
}
