mod color;
mod ellipse;
mod fill;
mod free_poly;
mod rect;
mod star;
mod stroke;
mod transform;

pub use self::{
    color::*, ellipse::*, fill::*, free_poly::*, rect::*, star::*, stroke::*, transform::*,
};

use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to convert free poly: {0}")]
    FreePolyInvalid(#[from] FreePolyError),
    #[error("Failed to convert rect: {0}")]
    RectInvalid(#[from] RectError),
    #[error("Failed to convert ellipse: {0}")]
    EllipseInvalid(#[from] EllipseError),
    #[error("Failed to convert star: {0}")]
    StarInvalid(#[from] StarError),
    #[error("Failed to convert fill: {0}")]
    FillInvalid(#[from] FillError),
    #[error("Failed to convert stroke: {0}")]
    StrokeInvalid(#[from] StrokeError),
    #[error("Failed to convert transform: {0}")]
    TransformInvalid(#[from] TransformError),
    #[error("Nested groups aren't implemented yet")]
    NestedGroup,
    #[error("Top-level shapes that aren't groups aren't implemented yet")]
    NotAGroup,
}

#[derive(Debug)]
pub enum Geometry {
    FreePoly(FreePoly),
    Rect(Rect),
    Ellipse(Ellipse),
    Star(Star),
    Null,
}

#[derive(Debug, Default)]
pub struct Style {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    // pub merge: Option<Merge>,
    // pub trim: Option<Trim>,
    // pub rounded_corners: Option<RoundedCorners>,
    pub transform: Option<Transform>,
}

#[derive(Debug)]
pub struct Shape {
    pub geometry: Geometry,
    pub style: Style,
}

fn politely_set<T: Debug>(dest: &mut Option<T>, val: T) {
    if let Some(existing) = dest {
        log::warn!(
            "ignoring `{:?}` because we already have `{:?}`; this is probably a bug",
            val,
            existing
        );
    } else {
        *dest = Some(val);
    }
}

impl Shape {
    fn from_group(
        group: bodymovin::shapes::Group,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Option<Self>, Error> {
        let mut geometry = None;
        let mut style = Style::default();
        for item in group.items {
            match item {
                // Geometry
                bodymovin::shapes::AnyShape::Shape(shape) => {
                    geometry = Some(Geometry::FreePoly(FreePoly::from_bodymovin(
                        shape,
                        frame_rate,
                        position_scale,
                        size_scale,
                    )?));
                }
                bodymovin::shapes::AnyShape::Rect(rect) => {
                    geometry = Some(Geometry::Rect(Rect::from_bodymovin(
                        rect,
                        frame_rate,
                        position_scale,
                        size_scale,
                    )?));
                }
                bodymovin::shapes::AnyShape::Ellipse(ellipse) => {
                    geometry = Some(Geometry::Ellipse(Ellipse::from_bodymovin(
                        ellipse,
                        frame_rate,
                        position_scale,
                        size_scale,
                    )?));
                }
                bodymovin::shapes::AnyShape::Star(star) => {
                    geometry = Some(Geometry::Star(Star::from_bodymovin(
                        star,
                        frame_rate,
                        position_scale,
                        size_scale[0], // TODO: How to select which axis to use for radius scaling?
                    )?));
                }

                // Style
                bodymovin::shapes::AnyShape::Fill(fill) => {
                    politely_set(&mut style.fill, Fill::from_bodymovin(fill, frame_rate)?);
                }
                bodymovin::shapes::AnyShape::GradientFill(gradient_fill) => {
                    politely_set(
                        &mut style.fill,
                        Fill::from_bodymovin_with_gradient(gradient_fill, frame_rate)?,
                    );
                }
                bodymovin::shapes::AnyShape::Stroke(stroke) => {
                    politely_set(
                        &mut style.stroke,
                        Stroke::from_bodymovin(stroke, frame_rate, size_scale[0])?, // TODO: Which axis to use for stroke scaling?
                    );
                }
                bodymovin::shapes::AnyShape::GradientStroke(gradient_stroke) => {
                    politely_set(
                        &mut style.stroke,
                        Stroke::from_bodymovin_with_gradient(gradient_stroke, frame_rate)?,
                    );
                }
                bodymovin::shapes::AnyShape::Merge(_merge) => {
                    log::warn!("merges aren't implemented yet; ignoring");
                    // politely_set(&mut style.merge, merge);
                }
                bodymovin::shapes::AnyShape::Trim(_trim) => {
                    log::warn!("trims aren't implemented yet; ignoring");
                    // politely_set(&mut style.trim, trim);
                }
                bodymovin::shapes::AnyShape::RoundedCorners(_rounded_corners) => {
                    log::warn!("rounded corners aren't implemented yet; ignoring");
                    // politely_set(&mut style.rounded_corners, rounded_corners);
                }
                bodymovin::shapes::AnyShape::Transform(transform) => {
                    politely_set(
                        &mut style.transform,
                        Transform::from_bodymovin_shape(transform, frame_rate, position_scale)?,
                    );
                }

                bodymovin::shapes::AnyShape::Group(_group) => {
                    // TODO: do we need to support this?
                    Err(Error::NestedGroup)?
                }

                bodymovin::shapes::AnyShape::Repeater(_) => Err(Error::NotAGroup)?,
            }
        }
        Ok(geometry.map(|geometry| Self { geometry, style }))
    }

    pub fn from_anyshape(
        shape: bodymovin::shapes::AnyShape,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Self, Error> {
        match shape {
            // Geometry
            bodymovin::shapes::AnyShape::Shape(shape) => Ok(Self {
                geometry: Geometry::FreePoly(FreePoly::from_bodymovin(
                    shape,
                    frame_rate,
                    position_scale,
                    size_scale,
                )?),
                style: Style::default(),
            }),
            bodymovin::shapes::AnyShape::Rect(rect) => Ok(Self {
                geometry: Geometry::Rect(Rect::from_bodymovin(
                    rect,
                    frame_rate,
                    position_scale,
                    size_scale,
                )?),
                style: Style::default(),
            }),
            bodymovin::shapes::AnyShape::Ellipse(ellipse) => Ok(Self {
                geometry: Geometry::Ellipse(Ellipse::from_bodymovin(
                    ellipse,
                    frame_rate,
                    position_scale,
                    size_scale,
                )?),
                style: Style::default(),
            }),
            bodymovin::shapes::AnyShape::Star(star) => Ok(Self {
                geometry: Geometry::Star(Star::from_bodymovin(
                    star,
                    frame_rate,
                    position_scale,
                    size_scale[0], // TODO: How to select which axis to use for radius scaling?
                )?),
                style: Style::default(),
            }),

            // Style
            bodymovin::shapes::AnyShape::Fill(fill) => Ok(Self {
                geometry: Geometry::Null,
                style: Style {
                    fill: Some(Fill::from_bodymovin(fill, frame_rate)?),
                    ..Style::default()
                },
            }),
            bodymovin::shapes::AnyShape::GradientFill(gradient_fill) => Ok(Self {
                geometry: Geometry::Null,
                style: Style {
                    fill: Some(Fill::from_bodymovin_with_gradient(
                        gradient_fill,
                        frame_rate,
                    )?),
                    ..Style::default()
                },
            }),
            bodymovin::shapes::AnyShape::Stroke(stroke) => Ok(Self {
                geometry: Geometry::Null,
                style: Style {
                    stroke: Some(Stroke::from_bodymovin(stroke, frame_rate, size_scale[0])?),
                    ..Style::default()
                },
            }),
            bodymovin::shapes::AnyShape::GradientStroke(gradient_stroke) => Ok(Self {
                geometry: Geometry::Null,
                style: Style {
                    stroke: Some(Stroke::from_bodymovin_with_gradient(
                        gradient_stroke,
                        frame_rate,
                    )?),
                    ..Style::default()
                },
            }),
            bodymovin::shapes::AnyShape::Merge(_merge) => {
                log::warn!("merges aren't implemented yet; ignoring");
                Ok(Self {
                    geometry: Geometry::Null,
                    style: Style::default(),
                })
            }
            bodymovin::shapes::AnyShape::Trim(_trim) => {
                log::warn!("trims aren't implemented yet; ignoring");
                Ok(Self {
                    geometry: Geometry::Null,
                    style: Style::default(),
                })
            }
            bodymovin::shapes::AnyShape::RoundedCorners(_rounded_corners) => {
                log::warn!("rounded corners aren't implemented yet; ignoring");
                Ok(Self {
                    geometry: Geometry::Null,
                    style: Style::default(),
                })
            }
            bodymovin::shapes::AnyShape::Transform(transform) => Ok(Self {
                geometry: Geometry::Null,
                style: Style {
                    transform: Some(Transform::from_bodymovin_shape(
                        transform,
                        frame_rate,
                        position_scale,
                    )?),
                    ..Style::default()
                },
            }),

            bodymovin::shapes::AnyShape::Group(group) => {
                Ok(Self::from_group(group, frame_rate, position_scale, size_scale)?.unwrap())
            }

            bodymovin::shapes::AnyShape::Repeater(_) => Err(Error::NotAGroup)?,
        }
    }

    pub(crate) fn from_bodymovin(
        shape: bodymovin::shapes::AnyShape,
        frame_rate: f64,
        position_scale: &Vec<f64>,
        size_scale: &Vec<f64>,
    ) -> Result<Option<Self>, Error> {
        match shape {
            bodymovin::shapes::AnyShape::Group(group) => {
                Self::from_group(group, frame_rate, position_scale, size_scale)
            }

            bodymovin::shapes::AnyShape::Shape(shape) => {
                println!("Shape!");
                Ok(Some(Self::from_anyshape(
                    bodymovin::shapes::AnyShape::Shape(shape),
                    frame_rate,
                    position_scale,
                    size_scale,
                )?))
            }
            bodymovin::shapes::AnyShape::Rect(rect) => {
                println!("Rect!");
                Ok(Some(Self::from_anyshape(
                    bodymovin::shapes::AnyShape::Rect(rect),
                    frame_rate,
                    position_scale,
                    size_scale,
                )?))
            }
            bodymovin::shapes::AnyShape::Ellipse(ellipse) => {
                println!("Ellipse!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::Star(star) => {
                println!("Star!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::Fill(fill) => {
                println!("Fill!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::GradientFill(gradient_fill) => {
                println!("GradientFill!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::GradientStroke(gradient_stroke) => {
                println!("GradientStroke!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::Stroke(stroke) => {
                println!("Stroke!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::Merge(merge) => {
                println!("Merge!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::Trim(trim) => {
                println!("Trim!");
                Ok(Some(Self::from_anyshape(
                    bodymovin::shapes::AnyShape::Trim(trim),
                    frame_rate,
                    position_scale,
                    size_scale,
                )?))
            }
            bodymovin::shapes::AnyShape::Repeater(repeater) => {
                println!("Repeater!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::RoundedCorners(rounded_corners) => {
                println!("RoundedCorners!");
                Err(Error::NotAGroup)
            }
            bodymovin::shapes::AnyShape::Transform(transform) => {
                println!("Transform!");
                Err(Error::NotAGroup)
            }
            // TODO: will this ever happen?
            // Note: Yes, it will
            _ => Err(Error::NotAGroup),
        }
    }
}
