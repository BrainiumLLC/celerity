use crate::after_effects::MaybeTrack;

#[derive(Debug)]
pub struct Ellipse {
    pub direction: f64,
    pub position: MaybeTrack<gee::Point<f64>, f64>,
    pub size: MaybeTrack<gee::Size<f64>, f64>,
}

impl Ellipse {
    pub(crate) fn from_bodymovin(ellipse: bodymovin::shapes::Ellipse, frame_rate: f64) -> Self {
        Self {
            direction: ellipse.direction,
            position: MaybeTrack::from_multi_dimensional(ellipse.position, frame_rate),
            size: MaybeTrack::from_multi_dimensional(ellipse.size, frame_rate),
        }
    }
}
