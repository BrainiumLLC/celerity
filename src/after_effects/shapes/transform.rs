use crate::{after_effects::MaybeTrack, Animation};
use time_point::Duration;

#[derive(Debug)]
pub struct Transform {
    anchor_point: MaybeTrack<gee::Vector<f64>, f64>,
    translation: MaybeTrack<gee::Vector<f64>, f64>,
    scale: MaybeTrack<gee::Vector<f64>, f64>,
    rotation: MaybeTrack<gee::Angle<f64>, f64>,
    opacity: MaybeTrack<f64>,
    skew: MaybeTrack<gee::Angle<f64>, f64>,
    skew_axis: MaybeTrack<f64>,
}

impl Transform {
    pub(crate) fn from_bodymovin_helper(
        transform: bodymovin::helpers::Transform,
        frame_rate: f64,
    ) -> Self {
        // TODO: we're not handling px/py/pz
        Self {
            anchor_point: MaybeTrack::from_multi_dimensional(transform.anchor_point, frame_rate),
            translation: MaybeTrack::from_multi_dimensional(transform.position, frame_rate),
            scale: MaybeTrack::from_multi_dimensional(transform.scale, frame_rate),
            rotation: MaybeTrack::from_value(transform.rotation, frame_rate),
            opacity: MaybeTrack::from_value(transform.opacity, frame_rate),
            skew: MaybeTrack::from_value(transform.skew, frame_rate),
            skew_axis: MaybeTrack::from_value(transform.skew_axis, frame_rate),
        }
    }

    pub(crate) fn from_bodymovin_shape(
        transform: bodymovin::shapes::Transform,
        frame_rate: f64,
    ) -> Self {
        Self {
            anchor_point: MaybeTrack::from_multi_dimensional(transform.anchor_point, frame_rate),
            translation: MaybeTrack::from_multi_dimensional(transform.position, frame_rate),
            scale: MaybeTrack::from_multi_dimensional(transform.scale, frame_rate),
            rotation: MaybeTrack::from_value(transform.rotation, frame_rate),
            opacity: MaybeTrack::from_value(transform.opacity, frame_rate),
            skew: MaybeTrack::from_value(transform.skew, frame_rate),
            skew_axis: MaybeTrack::from_value(transform.skew_axis, frame_rate),
        }
    }

    pub fn sample_transform(&self, elapsed: Duration) -> gee::Transform<f64> {
        let anchor_point = self.anchor_point.sample(elapsed);
        let translation = self.translation.sample(elapsed);
        let rotation = self.rotation.sample(elapsed);
        let skew = self.skew.sample(elapsed);
        // scale is a percentage!
        let scale = self.scale.sample(elapsed) / gee::Vector::uniform(100.0);
        log::info!(
            "sampled transform components: translation `{:?}`, rotation `{:?}`, skew `{:?}`, and scale `{:?}`",
            translation,
            rotation,
            skew,
            scale,
        );
        // TODO: what to do with skew_axis?
        gee::Transform::from_decomposition(translation, rotation, skew, scale)
            .pre_translate_vector(-anchor_point)
    }

    pub fn sample_opacity(&self, elapsed: Duration) -> f64 {
        let opacity = self.opacity.sample(elapsed) / 100.0;
        log::info!("sampled opacity {:?}", opacity);
        opacity
    }
}
