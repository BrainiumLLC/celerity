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

#[derive(Debug)]
pub enum Geometry {
    FreePoly(FreePoly),
    Rect(Rect),
    Ellipse(Ellipse),
    Star(Star),
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
        log::error!(
            "ignoring `{:?}` because we already have `{:?}`; this is probably a bug",
            val,
            existing
        );
    } else {
        *dest = Some(val);
    }
}

impl Shape {
    fn from_group(group: bodymovin::shapes::Group, frame_rate: f64) -> Option<Self> {
        let (geometry, style) = group.items.into_iter().fold(
            (None, Style::default()),
            |(mut geometry, mut style), item| {
                match item {
                    // Geometry
                    bodymovin::shapes::AnyShape::Shape(shape) => {
                        geometry = Some(Geometry::FreePoly(FreePoly::from_bodymovin(
                            shape, frame_rate,
                        )));
                    }
                    bodymovin::shapes::AnyShape::Rect(rect) => {
                        geometry = Some(Geometry::Rect(Rect::from_bodymovin(rect, frame_rate)));
                    }
                    bodymovin::shapes::AnyShape::Ellipse(ellipse) => {
                        geometry = Some(Geometry::Ellipse(Ellipse::from_bodymovin(
                            ellipse, frame_rate,
                        )));
                    }
                    bodymovin::shapes::AnyShape::Star(star) => {
                        geometry = Some(Geometry::Star(Star::from_bodymovin(star, frame_rate)));
                    }

                    // Style
                    bodymovin::shapes::AnyShape::Fill(fill) => {
                        politely_set(&mut style.fill, Fill::from_bodymovin(fill, frame_rate));
                    }
                    bodymovin::shapes::AnyShape::GradientFill(gradient_fill) => {
                        politely_set(
                            &mut style.fill,
                            Fill::from_bodymovin_with_gradient(gradient_fill, frame_rate),
                        );
                    }
                    bodymovin::shapes::AnyShape::Stroke(stroke) => {
                        politely_set(
                            &mut style.stroke,
                            Stroke::from_bodymovin(stroke, frame_rate),
                        );
                    }
                    bodymovin::shapes::AnyShape::GradientStroke(gradient_stroke) => {
                        politely_set(
                            &mut style.stroke,
                            Stroke::from_bodymovin_with_gradient(gradient_stroke, frame_rate),
                        );
                    }
                    bodymovin::shapes::AnyShape::Merge(merge) => {
                        log::error!("merges aren't implemented yet; ignoring");
                        // politely_set(&mut style.merge, merge);
                    }
                    bodymovin::shapes::AnyShape::Trim(trim) => {
                        log::error!("trims aren't implemented yet; ignoring");
                        // politely_set(&mut style.trim, trim);
                    }
                    bodymovin::shapes::AnyShape::RoundedCorners(rounded_corners) => {
                        log::error!("rounded corners aren't implemented yet; ignoring");
                        // politely_set(&mut style.rounded_corners, rounded_corners);
                    }
                    bodymovin::shapes::AnyShape::Transform(transform) => {
                        politely_set(
                            &mut style.transform,
                            Transform::from_bodymovin_shape(transform, frame_rate),
                        );
                    }

                    // Hopefully we don't have to worry about this...
                    bodymovin::shapes::AnyShape::Group(group) => {
                        log::error!("nested groups aren't implemented; ignoring");
                    }
                }
                (geometry, style)
            },
        );
        geometry.map(|geometry| Self { geometry, style })
    }

    pub(crate) fn from_bodymovin(
        shape: bodymovin::shapes::AnyShape,
        frame_rate: f64,
    ) -> Option<Self> {
        match shape {
            bodymovin::shapes::AnyShape::Group(group) => Self::from_group(group, frame_rate),
            _ => todo!(),
        }
    }
}
