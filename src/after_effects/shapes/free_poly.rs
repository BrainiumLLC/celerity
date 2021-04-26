use thiserror::Error;

#[derive(Debug, Error)]
pub enum FreePolyError {}

#[derive(Debug)]
pub struct FreePoly {
    pub direction: f64,
    // pub vertices: MaybeTrack<FreePolyProp, f64>,
}

impl FreePoly {
    pub(crate) fn from_bodymovin(
        shape: bodymovin::shapes::Shape,
        frame_rate: f64,
    ) -> Result<Self, FreePolyError> {
        log::warn!("free polygons aren't implemented yet");
        Ok(Self {
            direction: shape.direction,
            // vertices: shape.vertices.into(),
        })
    }
}
