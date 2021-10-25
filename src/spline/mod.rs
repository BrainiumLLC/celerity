pub mod bezier;
pub mod bezier_ease;
pub mod bezier_path;
pub mod catmull_rom;

use self::{bezier::dt_cubic_bezier, catmull_rom::catmull_rom_value};
use crate::{lerp::linear_value, Animatable};
use gee::en::num_traits::Zero as _;

// Spline polyline subdivision
const SPLINE_SUBDIVISION: usize = 64;

// Map from time to distance
#[derive(Clone, Debug)]
pub struct SplineMap {
    // Animatable always lerps using f64, and distance is always an f64
    pub steps: Vec<(f64, f64)>,
    pub length: f64,
    rectify: bool,
}

// Look up linear easing by arc length using a spline map

pub fn spline_ease(spline_map: &SplineMap, t: f64) -> f64 {
    spline_map
        .rectify
        .then(|| {
            // Convert t 0..1 to arc length 0..d
            let elapsed_distance = t * spline_map.length;

            // Find closest interval
            let len = spline_map.steps.len();
            let i = usize::min(len - 2, find_index(spline_map, elapsed_distance));
            let start = spline_map.steps[i];
            let end = spline_map.steps[i + 1];

            // Use chordal catmull-rom if we have a window of 4 steps.
            // This reduces jitter by an order of magnitude.
            if i > 0 && i < spline_map.steps.len() - 2 {
                let prev = spline_map.steps[i - 1];
                let next = spline_map.steps[i + 2];
                catmull_rom_value(
                    &prev.0,
                    &start.0,
                    &end.0,
                    &next.0,
                    prev.1,
                    start.1,
                    end.1,
                    next.1,
                    elapsed_distance,
                )
            } else {
                // Lerp steps[i] and steps[i+1]
                // (will only be used if easing beyond the start or end)
                linear_value(&start.0, &end.0, start.1, end.1, elapsed_distance)
            }
        })
        .unwrap_or(t)
}

// Find index for lookup in spline map with binary search.
// Returns last index with d < distance.
pub fn find_index(spline_map: &SplineMap, distance: f64) -> usize {
    let len = spline_map.steps.len();
    let mut size = len;
    let mut base: usize = 0;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let step = spline_map.steps[mid];
        if step.1 < distance {
            base = mid
        }
        size -= half;
    }
    base
}

impl SplineMap {
    // Make a spline map to map "spline time" 0..1 to arc length 0..d.
    // Integrates with Euler's rule.

    pub fn from_spline<V: Animatable, F: Fn(f64) -> V>(f: F, rectify: bool) -> SplineMap {
        let mut steps = Vec::new();
        let mut length: f64 = 0.0;
        let mut point = f(0.0);

        // Insert one negative point before
        let step = 1.0 / (SPLINE_SUBDIVISION as f64);
        steps.push((-step, -f(-step).distance_to(point)));
        steps.push((0.0, 0.0));

        // Measure arc length of each segment
        let mut t = 0.0;
        for _i in 0..=SPLINE_SUBDIVISION {
            t += step;
            let next = f(t);
            let d = next.distance_to(point);
            point = next;

            length += d;
            steps.push((t, length));
        }

        // Ignore one point after
        length = steps[steps.len() - 2].1;

        SplineMap {
            steps,
            length,
            rectify,
        }
    }

    // Make a spline map from a cubic bezier to map "spline time" 0..1 to arc length 0..d.
    // Uses analytic derivatives and simpson's rule for more accurate integration.
    // (only makes a difference for strongly cusped curves)

    pub fn from_bezier<V: Animatable>(b0: &V, b1: &V, b2: &V, b3: &V, rectify: bool) -> SplineMap {
        let mut steps = Vec::new();
        let mut length: f64 = 0.0;
        let zero = b0.map(|_| V::Component::zero());

        // Integrate arc length between t = a..b
        let integrate = |a, b| {
            // Get tangents at start, middle and end
            let dt0 = dt_cubic_bezier(b0, b1, b2, b3, a);
            let dt1 = dt_cubic_bezier(b0, b1, b2, b3, (a + b) / 2.0);
            let dt2 = dt_cubic_bezier(b0, b1, b2, b3, b);

            // Get magnitude
            let ds0 = zero.distance_to(dt0);
            let ds1 = zero.distance_to(dt1);
            let ds2 = zero.distance_to(dt2);

            // Simpson's 1/3 rule
            (ds0 + 4.0 * ds1 + ds2) * (b - a) / 6.0
        };

        // Insert one negative point before
        let step = 1.0 / (SPLINE_SUBDIVISION as f64);
        steps.push((-step, -integrate(-step, 0.0)));
        steps.push((0.0, 0.0));

        // Measure arc length of each segment
        let mut last = 0.0;
        let mut t = 0.0;
        for _i in 0..=SPLINE_SUBDIVISION {
            t += step;
            let d = integrate(last, t);
            last = t;

            length += d;
            steps.push((t, length));
        }

        // Ignore one point after
        length = steps[steps.len() - 2].1;

        SplineMap {
            steps,
            length,
            rectify,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{bezier::cubic_bezier, *};
    use gee::en::Num as _;

    const MATCH_TOLERANCE: f64 = 1e-3;

    pub fn approx_eq(lhs: f64, rhs: f64, epsilon: f64) -> bool {
        lhs.is_finite() && rhs.is_finite() && ((lhs - epsilon)..(lhs + epsilon)).contains(&rhs)
    }

    pub fn integrate_length<V: Animatable, F: Fn(f64) -> V>(
        from: f64,
        to: f64,
        divide: usize,
        f: F,
    ) -> f64 {
        let mut length: f64 = 0.0;
        let mut point = f(from);

        let mut t = from;
        let step = (to - from) / (divide as f64);

        // Measure arc length of each segment
        for _i in 0..divide {
            t += step;
            let next = f(t);
            let d = next.distance_to(point);

            length += d;
            point = next;
        }
        length
    }

    #[test]
    fn test_maps_match() {
        let b0 = (0f64, 0f64);
        let b1 = (50f64, -50f64);
        let b2 = (100f64, 100f64);
        let b3 = (150f64, 0f64);
        let spline_map = SplineMap::from_spline(|t| cubic_bezier(&b0, &b1, &b2, &b3, t), true);
        let bezier_map = SplineMap::from_bezier(&b0, &b1, &b2, &b3, true);
        let tolerance = MATCH_TOLERANCE * spline_map.length;
        let mut i = 0;
        for &s1 in spline_map.steps.iter() {
            let s2 = bezier_map.steps[i];
            i += 1;
            println!("{:?} {:?}", s1, s2);
            assert!(approx_eq(s1.0, s2.0, MATCH_TOLERANCE));
            assert!(approx_eq(s1.1, s2.1, tolerance));
        }
    }

    #[test]
    fn test_spline_map() {
        let b0 = (0f64, 0f64);
        let b1 = (50f64, -50f64);
        let b2 = (100f64, 100f64);
        let b3 = (150f64, 0f64);
        let spline_map = SplineMap::from_spline(|t| cubic_bezier(&b0, &b1, &b2, &b3, t), true);

        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 100, 1e-3);
        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 20, 1e-3);
    }

    #[test]
    fn test_bezier_map() {
        let b0 = (0f64, 0f64);
        let b1 = (50f64, -50f64);
        let b2 = (100f64, 100f64);
        let b3 = (150f64, 0f64);
        let spline_map = SplineMap::from_bezier(&b0, &b1, &b2, &b3, true);

        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 100, 1e-3);
        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 20, 1e-3);
    }

    #[test]
    fn test_spline_map_cusp() {
        let b0 = (0f64, 0f64);
        let b1 = (50f64, -50f64);
        let b2 = (350f64, 100f64);
        let b3 = (150f64, 0f64);
        let spline_map = SplineMap::from_spline(|t| cubic_bezier(&b0, &b1, &b2, &b3, t), true);

        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 100, 3e-2);
        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 20, 3e-2);
    }

    #[test]
    fn test_bezier_map_cusp() {
        let b0 = (0f64, 0f64);
        let b1 = (50f64, -50f64);
        let b2 = (350f64, 100f64);
        let b3 = (150f64, 0f64);
        let spline_map = SplineMap::from_bezier(&b0, &b1, &b2, &b3, true);

        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 100, 1e-2);
        test_with_spline_map(&spline_map, &b0, &b1, &b2, &b3, 20, 1e-2);
    }

    fn test_with_spline_map(
        spline_map: &SplineMap,
        b0: &(f64, f64),
        b1: &(f64, f64),
        b2: &(f64, f64),
        b3: &(f64, f64),
        subdivision: usize,
        tolerance: f64,
    ) {
        // Divide spline into N equal segments of arc length
        let step = spline_map.length / (subdivision as f64);
        let epsilon = tolerance;
        println!("Expected step size {}", step);

        // Check if each segment is the same length
        let mut min = 1000.0;
        let mut max = -1000.0;

        for i in 0..subdivision {
            let t1 = i.to_f64() / subdivision.to_f64();
            let t2 = (i + 1).to_f64() / subdivision.to_f64();

            let ease1 = spline_ease(spline_map, t1);
            let ease2 = spline_ease(spline_map, t2);

            // Get ground truth arc length
            let d = integrate_length(ease1, ease2, 128, |t| cubic_bezier(b0, b1, b2, b3, t));
            let error = ((d - step) / step).abs();

            // Track min/max error
            min = f64::min(min, error);
            max = f64::max(max, error);

            let warn = if error > epsilon { "⚠️" } else { "" };
            println!(
                "t = {:.2}..{:.2}  ±{:.6} ±{:.3}% {}",
                t1,
                t2,
                (d - step).abs(),
                error * 100.0,
                warn
            );
        }
        println!("error min: {:.6}% max: {:.6}%", min * 100.0, max * 100.0);
        assert!(approx_eq(0.0, max, epsilon));
    }
}
