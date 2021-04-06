mod conv;
pub mod layers;
mod maybe_track;
pub mod shapes;

use self::layers::Layer;
pub use self::maybe_track::MaybeTrack;
use crate::interval::BezierEase;
pub use bodymovin;
use std::{fmt::Debug, path::Path};

impl From<bodymovin::properties::Bezier1d> for BezierEase {
    fn from(bezier: bodymovin::properties::Bezier1d) -> Self {
        Self {
            ox: bezier.out_value.x,
            oy: bezier.out_value.y,
            ix: bezier.in_value.x,
            iy: bezier.in_value.y,
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub size: gee::Size<i64>,
    pub layers: Vec<Layer>,
}

impl Scene {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, bodymovin::Error> {
        bodymovin::Bodymovin::load(path).map(Self::from_bodymovin)
    }

    pub fn from_bodymovin(
        bodymovin::Bodymovin {
            width,
            height,
            frame_rate,
            layers,
            ..
        }: bodymovin::Bodymovin,
    ) -> Self {
        Self {
            size: gee::Size::new(width, height),
            layers: layers
                .into_iter()
                .map(|layer| Layer::from_bodymovin(layer, frame_rate))
                .collect(),
        }
    }
}
