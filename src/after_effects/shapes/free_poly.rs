use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreePolyError {}

#[derive(Debug)]
pub struct FreePoly {
    pub direction: f64,
    // pub vertices: MaybeTrack<FreePolyProp>,
}

impl FreePoly {
    pub(crate) fn from_bodymovin(
        shape: bodymovin::shapes::Shape,
        _frame_rate: f64,
        _position_scale: &Vec<f64>,
        _size_scale: &Vec<f64>,
    ) -> Result<Self, FreePolyError> {
        log::warn!("free polygons aren't implemented yet");
        Ok(Self {
            direction: shape.direction.unwrap_or(0.0),
            // vertices: shape.vertices.into(),
        })
    }
}
