use crate::after_effects::shapes;

#[derive(Debug)]
pub struct Shape {
    pub transform: shapes::Transform,
    pub shapes: Vec<shapes::Shape>,
}

impl Shape {
    pub(crate) fn from_bodymovin(layer: bodymovin::layers::Shape, frame_rate: f64) -> Self {
        Self {
            transform: shapes::Transform::from_bodymovin_helper(layer.transform, frame_rate),
            shapes: layer
                .shapes
                .into_iter()
                // TODO: what to do with the options here?
                .flat_map(|shape| shapes::Shape::from_bodymovin(shape, frame_rate))
                .collect(),
        }
    }
}
