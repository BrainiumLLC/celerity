mod conv;
pub mod layers;
mod maybe_track;
pub mod shapes;

use self::layers::{Layer, ResizeOption};
pub use self::maybe_track::MaybeTrack;
use crate::component_wise::ComponentWise;
pub use bodymovin;
use gee::Size;
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
    pub fn load(
        path: impl AsRef<Path>,
        canvas_size: Size<f64>,
        size_scale: ResizeOption,
    ) -> Result<Self, Error> {
        bodymovin::Bodymovin::load(path)
            .map_err(Error::from)
            .and_then(|bodymovin| Self::from_bodymovin(bodymovin, canvas_size, size_scale))
    }

    pub fn from_bytes(
        bytes: impl AsRef<[u8]>,
        canvas_size: Size<f64>,
        size_scale: ResizeOption,
    ) -> Result<Self, Error> {
        bodymovin::Bodymovin::from_bytes(bytes)
            .map_err(Error::from)
            .and_then(|bodymovin| Self::from_bodymovin(bodymovin, canvas_size, size_scale))
    }

    pub fn from_bodymovin(
        bodymovin::Bodymovin {
            width,
            height,
            frame_rate,
            layers,
            ..
        }: bodymovin::Bodymovin,
        canvas_size: Size<f64>,
        size_scale: ResizeOption,
    ) -> Result<Self, Error> {
        let (position_scale, size_scale) =
            Self::calculate_scales2d(Size::new(width, height).to_f64(), canvas_size, size_scale);

        layers
            .into_iter()
            .map(|layer| Layer::from_bodymovin(layer, frame_rate, &position_scale, &size_scale))
            .collect::<Result<_, _>>()
            .map(|layers| Self {
                size: canvas_size.to_i64(),
                layers,
            })
            .map_err(Error::from)
    }

    pub fn calculate_scales2d(
        export_size: Size<f64>,
        canvas_size: Size<f64>,
        size_scale: ResizeOption,
    ) -> (Vec<f64>, Vec<f64>) {
        let rel_size = canvas_size.zip_map(export_size, |c, e| c / e);
        let position_scale = vec![rel_size.width, rel_size.height];
        let size_scale = match size_scale {
            ResizeOption::UseWidth => vec![position_scale[0]; position_scale.len()],
            ResizeOption::UseHeight => vec![position_scale[1]; position_scale.len()],
            ResizeOption::UseDepth => vec![position_scale[2]; position_scale.len()],
            ResizeOption::UseLargest => {
                vec![position_scale.iter().fold(0.0, |a, b| f64::max(a, *b)); position_scale.len()]
            }
            ResizeOption::UseSmallest => {
                vec![position_scale.iter().fold(0.0, |a, b| f64::min(a, *b)); position_scale.len()]
            }
            ResizeOption::UseAll => position_scale.clone(),
        };

        (position_scale, size_scale)
    }
}
