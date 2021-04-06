use crate::{after_effects::MaybeTrack, Animation as _};
use time_point::Duration;

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
    ) -> Self {
        match ty {
            bodymovin::shapes::GradientType::Linear => {
                if highlight_length.is_some() || highlight_angle.is_some() {
                    log::warn!(
                        "gradient specifies a highlight length or angle despite being linear"
                    );
                }
                Self::Linear
            }
            bodymovin::shapes::GradientType::Radial => {
                // TODO: don't unwrap
                highlight_length
                    .zip(highlight_angle)
                    .map(|(highlight_length, highlight_angle)| Self::Radial {
                        highlight_length: MaybeTrack::from_value(highlight_length, frame_rate),
                        highlight_angle: MaybeTrack::from_value(highlight_angle, frame_rate),
                    })
                    .expect(
                        "gradient didn't specify a highlight length or angle despite being radial",
                    )
            }
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
    ) -> Self {
        Self::Solid(MaybeTrack::from_multi_dimensional(color, frame_rate))
    }

    pub(crate) fn from_bodymovin_gradient(
        start_point: bodymovin::properties::EitherMultiDimensional,
        end_point: bodymovin::properties::EitherMultiDimensional,
        ty: bodymovin::shapes::GradientType,
        highlight_length: Option<bodymovin::properties::EitherValue>,
        highlight_angle: Option<bodymovin::properties::EitherValue>,
        // color: ???,
        frame_rate: f64,
    ) -> Self {
        log::warn!("gradient colors aren't implemented yet");
        Self::Gradient(Gradient {
            start_point: MaybeTrack::from_multi_dimensional(start_point, frame_rate),
            end_point: MaybeTrack::from_multi_dimensional(end_point, frame_rate),
            ty: GradientType::from_bodymovin(ty, highlight_length, highlight_angle, frame_rate),
        })
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
