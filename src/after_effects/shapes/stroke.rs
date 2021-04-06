use crate::{
    after_effects::{shapes::Color, MaybeTrack},
    Animation as _,
};
pub use bodymovin::helpers::{LineCap, LineJoin};
use time_point::Duration;

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
    pub(crate) fn from_bodymovin(stroke: bodymovin::shapes::Stroke, frame_rate: f64) -> Self {
        Self {
            line_cap: stroke.line_cap,
            line_join: stroke.line_join,
            miter_limit: stroke.miter_limit,
            opacity: MaybeTrack::from_value(stroke.opacity, frame_rate),
            width: MaybeTrack::from_value(stroke.width, frame_rate),
            color: Color::from_bodymovin_solid(stroke.color, frame_rate),
        }
    }

    pub(crate) fn from_bodymovin_with_gradient(
        stroke: bodymovin::shapes::GradientStroke,
        frame_rate: f64,
    ) -> Self {
        Self {
            line_cap: stroke.line_cap,
            line_join: stroke.line_join,
            miter_limit: stroke.miter_limit,
            opacity: MaybeTrack::from_value(stroke.opacity, frame_rate),
            width: MaybeTrack::from_value(stroke.stroke_width, frame_rate),
            color: Color::from_bodymovin_gradient(
                stroke.start_point,
                stroke.end_point,
                stroke.ty,
                stroke.highlight_length,
                stroke.highlight_angle,
                frame_rate,
            ),
        }
    }

    pub fn sample_width(&self, elapsed: Duration) -> f64 {
        self.width.sample(elapsed)
    }
}
