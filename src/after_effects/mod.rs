mod conv;
pub mod layers;
mod maybe_track;
pub mod shapes;

use self::layers::Layer;
pub use self::maybe_track::MaybeTrack;
pub use bodymovin;
use std::{fmt::Debug, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to load exported animation: {0}")]
    ParseFailed(#[from] bodymovin::Error),
    #[error(transparent)]
    LayerInvalid(#[from] layers::LayerError),
}

#[derive(Debug)]
pub struct Scene {
    pub size: gee::Size<i64>,
    pub layers: Vec<Layer>,
}

impl Scene {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        bodymovin::Bodymovin::load(path)
            .map_err(Error::from)
            .and_then(Self::from_bodymovin)
    }

    pub fn from_bodymovin(
        bodymovin::Bodymovin {
            width,
            height,
            frame_rate,
            layers,
            ..
        }: bodymovin::Bodymovin,
    ) -> Result<Self, Error> {
        layers
            .into_iter()
            .map(|layer| Layer::from_bodymovin(layer, frame_rate))
            .collect::<Result<_, _>>()
            .map(|layers| Self {
                size: gee::Size::new(width, height),
                layers,
            })
            .map_err(Error::from)
    }
}
