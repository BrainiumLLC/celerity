// Spline polyline subdivision
const SPLINE_SUBDIVISION: usize = 64;
// Map from time to distance
#[derive(Debug)]
pub struct SplineMap/*<T, S>*/ {
    // Animatable always lerps using f64, and distance is always an f64
    steps: Vec<(f64, f64)>,
    length: f64,
}

// Look up easing using a spline map
pub fn spline_ease(
    spline_map: &SplineMap,
    t: f64
) -> f64 {
    // Convert t 0..1 to arc length 0..d
    let d = t * spline_map.length;
    // Do binary search to find closest interval (optimize this)
    let i = spline_map.steps.iter().enumerate().find(|(_i, step)| step.1 > d).unwrap_or((spline_map.steps.len(), &(0f64, 0f64))).0 - 1;
    let start = spline_map.steps[i];
    let end   = spline_map.steps[usize::min(i + 1, spline_map.steps.len() - 1)];
    // Lerp between steps[i].t  and steps[i+1].t
    let elapsed_distance = spline_map.steps[0].distance_to(start);
    let step_length = start.distance_to(end);
    let step_distance = d - elapsed_distance;
    let relative_distance = step_distance / step_length;

    let t = start.0.lerp(end.0, relative_distance);
    
    t
}

// Make a spline map to map "spline time" 0..1 to arc length 0..d
pub fn make_spline_map<V: Animatable<C>, C: en::Num, F: Fn(f64) -> V>
(
    f: F
) -> SplineMap {
    let mut steps = Vec::new();
    let mut length: f64 = 0.0;
    let mut point = f(length);
    steps.push((0.0, 0.0));
    // Measure arc length of each segment
    for i in 0..=SPLINE_SUBDIVISION {
        let t = (i as f64) / (SPLINE_SUBDIVISION as f64);
        let next = f(t);    
        let d = next.distance_to(point);
        length += d;
        steps.push((t, length));
        point = next;
    }
    return SplineMap { steps, length };
}

// TODO: proper spline test
fn test_spline_map () {
    let b0 = (0f64, 0f64);
    let b1 = (50f64, 50f64);
    let b2 = (100f64, 50f64);
    let b3 = (150f64, 0f64);

    let spline_map = make_spline_map(|t| cubic_bezier(b0, b1, b2, b3, t));

    let mut previous = cubic_bezier(b0, b1, b2, b3, 0f64);
    let mut previous_distance = 0f64;
    for i in 0..=SPLINE_SUBDIVISION {
        let t = en::cast::<f64, _>(i) / en::cast::<f64, _>(SPLINE_SUBDIVISION);
        let position = cubic_bezier(b0, b1, b2, b3, t);
        let ease = spline_ease(&spline_map, t);
        let eased_position = cubic_bezier(b0, b1, b2, b3, ease);
        let difference = position.distance_to(eased_position);
        let rectified_distance = previous.distance_to(eased_position);
        previous = eased_position;
        println!("{} \\ {} : {:?} \\ {:?} -- {:?}", t, ease, position, eased_position, rectified_distance);
    }
}