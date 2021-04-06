use crate::after_effects::{shapes::Color, MaybeTrack};

#[derive(Debug)]
pub struct Fill {
    pub opacity: MaybeTrack<f64>,
    pub color: Color,
}

impl Fill {
    pub(crate) fn from_bodymovin(fill: bodymovin::shapes::Fill, frame_rate: f64) -> Self {
        Self {
            opacity: MaybeTrack::from_value(fill.opacity, frame_rate),
            color: Color::from_bodymovin_solid(fill.color, frame_rate),
        }
    }

    pub(crate) fn from_bodymovin_with_gradient(
        fill: bodymovin::shapes::GradientFill,
        frame_rate: f64,
    ) -> Self {
        Self {
            opacity: MaybeTrack::from_value(fill.opacity, frame_rate),
            color: Color::from_bodymovin_gradient(
                fill.start_point,
                fill.end_point,
                fill.ty,
                fill.highlight_length,
                fill.highlight_angle,
                frame_rate,
            ),
        }
    }
}
