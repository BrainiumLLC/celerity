use crate::{after_effects::MaybeTrack, Animation};
use time_point::Duration;

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
    ) -> Self {
        match ty {
            bodymovin::shapes::StarType::Star => {
                if let Some((inner_radius, inner_roundness)) = inner_radius.zip(inner_roundness) {
                    Self::Star {
                        inner_radius: MaybeTrack::from_value(inner_radius, frame_rate),
                        inner_roundness: MaybeTrack::from_value(inner_roundness, frame_rate),
                    }
                } else {
                    // TODO: don't panic
                    panic!("star doesn't specify an inner radius or innner roundness despite being a star-type star");
                }
            }
            bodymovin::shapes::StarType::Polygon => {
                if inner_radius.is_some() || inner_roundness.is_some() {
                    log::warn!("star specifies an inner radius or inner roundness despite being a polygon-type star")
                }
                Self::Polygon
            }
        }
    }
}

#[derive(Debug)]
pub struct Star {
    pub direction: f64,
    pub position: MaybeTrack<gee::Point<f64>, f64>,
    pub outer_radius: MaybeTrack<f64>,
    pub outer_roundness: MaybeTrack<f64>,
    pub rotation: MaybeTrack<gee::Angle<f64>, f64>,
    pub points: MaybeTrack<u32>,
    pub ty: StarType,
}

impl Star {
    pub(crate) fn from_bodymovin(star: bodymovin::shapes::Star, frame_rate: f64) -> Self {
        Self {
            direction: star.direction,
            position: MaybeTrack::from_multi_dimensional(star.position, frame_rate),
            outer_radius: MaybeTrack::from_value(star.outer_radius, frame_rate),
            outer_roundness: MaybeTrack::from_value(star.outer_roundness, frame_rate),
            rotation: MaybeTrack::from_value(star.rotation, frame_rate),
            points: MaybeTrack::from_value(star.points, frame_rate),
            ty: StarType::from_bodymovin(
                star.ty,
                star.inner_radius,
                star.inner_roundness,
                frame_rate,
            ),
        }
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
