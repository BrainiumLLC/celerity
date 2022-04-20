pub mod image;
pub mod null;
pub mod precomp;
pub mod shape;
pub mod solid;
pub mod text;

pub use self::{image::*, null::*, precomp::*, shape::*, solid::*, text::*};
use crate::component_wise::ComponentWise;
use bodymovin::layers::AnyLayer;
use gee::Size;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LayerError {
    #[error("Failed to convert precomp layer: {0}")]
    PreCompInvalid(#[from] PreCompError),
    #[error("Failed to convert image layer: {0}")]
    ImageInvalid(#[from] ImageError),
    #[error("Failed to convert solid layer: {0}")]
    SolidInvalid(#[from] SolidError),
    #[error("Failed to convert null layer: {0}")]
    NullInvalid(#[from] NullError),
    #[error("Failed to convert shape layer: {0}")]
    ShapeInvalid(#[from] ShapeError),
    #[error("Failed to convert text layer: {0}")]
    TextInvalid(#[from] TextError),
    #[error("Unimplemented layer type: {0:?}")]
    Unimplemented(bodymovin::layers::AnyLayer),
}

#[derive(Debug)]
pub enum ResizeOption {
    UseWidth,
    UseHeight,
    UseDepth,
    UseSmallest,
    UseLargest,
    UseAll,
}

#[derive(Debug)]
pub enum Layer {
    PreComp(PreComp),
    Solid(Solid),
    Image(Image),
    Null(Null),
    Shape(Shape),
    Text(Text),
}

impl Layer {
    pub(crate) fn from_bodymovin(
        layer: bodymovin::layers::AnyLayer,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Self, LayerError> {
        match layer {
            AnyLayer::PreComp(layer) => {
                PreComp::from_bodymovin(layer, frame_rate, &position_scale, &size_scale)
                    .map(Self::PreComp)
                    .map_err(LayerError::from)
            }
            AnyLayer::Solid(layer) => {
                Solid::from_bodymovin(layer, frame_rate, &position_scale, &size_scale)
                    .map(Self::Solid)
                    .map_err(LayerError::from)
            }
            AnyLayer::Image(layer) => {
                Image::from_bodymovin(layer, frame_rate, &position_scale, &size_scale)
                    .map(Self::Image)
                    .map_err(LayerError::from)
            }
            AnyLayer::Null(layer) => {
                Null::from_bodymovin(layer, frame_rate, &position_scale, &size_scale)
                    .map(Self::Null)
                    .map_err(LayerError::from)
            }
            AnyLayer::Shape(layer) => {
                Shape::from_bodymovin(layer, frame_rate, &position_scale, &size_scale)
                    .map(Self::Shape)
                    .map_err(LayerError::from)
            }
            AnyLayer::Text(layer) => {
                Text::from_bodymovin(layer, frame_rate, &position_scale, &size_scale)
                    .map(Self::Text)
                    .map_err(LayerError::from)
            }
            _ => Err(LayerError::Unimplemented(layer)),
        }
    }
}
