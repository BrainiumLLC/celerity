#[derive(Debug)]
pub struct FreePoly {
    pub direction: f64,
    // pub vertices: MaybeTrack<FreePolyProp, f64>,
}

impl FreePoly {
    pub(crate) fn from_bodymovin(shape: bodymovin::shapes::Shape, frame_rate: f64) -> Self {
        log::error!("free polygons aren't implemented yet");
        Self {
            direction: shape.direction,
            // vertices: shape.vertices.into(),
        }
    }
}
