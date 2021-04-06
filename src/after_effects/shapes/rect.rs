use crate::{after_effects::MaybeTrack, Animation};
use time_point::Duration;

#[derive(Debug)]
pub struct Rect {
    pub direction: f64,
    pub position: MaybeTrack<gee::Point<f64>, f64>,
    pub size: MaybeTrack<gee::Size<f64>, f64>,
    pub rounded_corners: MaybeTrack<f64>,
}

impl Rect {
    pub(crate) fn from_bodymovin(rect: bodymovin::shapes::Rect, frame_rate: f64) -> Self {
        Self {
            direction: rect.direction,
            position: MaybeTrack::from_multi_dimensional(rect.position, frame_rate),
            size: MaybeTrack::from_multi_dimensional(rect.size, frame_rate),
            rounded_corners: MaybeTrack::from_value(rect.rounded_corners, frame_rate),
        }
    }

    pub fn sample_rect(&self, elapsed: Duration) -> gee::Rect<f64> {
        gee::Rect::from_center(self.position.sample(elapsed), self.size.sample(elapsed))
    }

    pub fn sample_rounded_corners(&self, elapsed: Duration) -> f64 {
        self.rounded_corners.sample(elapsed)
    }
}
