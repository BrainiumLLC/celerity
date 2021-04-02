use gee::en;

use crate::Animatable;

// Newton-Raphson iterations
const NR_ITERATIONS: usize = 3;

pub fn cubic_bezier_ease(ox: f64, oy: f64, ix: f64, iy: f64, t: f64) -> f64 {
    // Uses a cubic 2D bezier curve to map linear interpolation time
    // to eased interpolation time.
    //
    // https://cubic-bezier.com
    //
    // Bezier control points:
    // [0, 0], [ox, oy], [ix, iy], [1, 1]
    //
    // The easing curve is parametric and
    // defined by (s ∈ [0...1]):
    //
    // x(s) = bezier X
    // y(s) = bezier Y
    //
    // The output is found by inverting x(s) and composing it with y(s):
    // out = y(x^-1(t))
    //
    // Note that Y is allowed to exceed [0...1] to allow under/overshoot
    // of 1D interpolation. But X is always monotonic (= invertible).
    //
    // Multiple cubic eases are typically chained so that the in
    // and out tangents line up at every keyframe.
    //
    // For 2D path-based animation, this temporal easing is applied
    // to each individual spatial spline segment, after rectifying it
    // by arc length. These motions never go backwards along the path,
    // so both X and Y will be monotonic.

    // Extend the curve beyond start and end by mirroring/flipping
    // This allows good central differences to be taken around start/end

    //if ox == ox && ix == iy { return t; }

    if t < 0.0 {
        -lookup(ox, oy, ix, iy, -t)
    } else if t > 1.0 {
        2.0 - lookup(ox, oy, ix, iy, 2.0 - t)
    } else {
        lookup(ox, oy, ix, iy, t)
    }
}

fn lookup(ox: f64, oy: f64, ix: f64, iy: f64, t: f64) -> f64 {
    fixed_bezier(oy, iy, invert_fixed_bezier(ox, ix, t))
}

fn square(x: f64) -> f64 {
    x * x
}
fn cube(x: f64) -> f64 {
    x * x * x
}

// Exact x(t) with fixed first and last control point
pub fn fixed_bezier(ox: f64, ix: f64, t: f64) -> f64 {
    let it = 1.0 - t;
    ox * 3.0 * square(it) * t + ix * 3.0 * it * square(t) + cube(t)
}

// Exact dx(t)/dt with fixed first and last control point
pub fn dt_fixed_bezier(ox: f64, ix: f64, t: f64) -> f64 {
    3.0 * (ox * (1.0 - t * (4.0 - 3.0 * t)) + t * (ix * (2.0 - 3.0 * t) + t))
}

// Approximate t = x^-1(x)
pub fn invert_fixed_bezier(ox: f64, ix: f64, x: f64) -> f64 {
    // Use Newton-Raphson iteration starting from the input time
    //
    // Converges O(1/n^2) almost everywhere,
    // except near horizontal tangents where it is O(1/n).
    let mut t = x;
    for _ in 1..=NR_ITERATIONS {
        let v = fixed_bezier(ox, ix, t) - x;
        let dvdt = dt_fixed_bezier(ox, ix, t);

        if v == 0.0 {
            break;
        }
        if dvdt == 0.0 {
            break;
        }

        t = t - v / dvdt;
    }

    t
}

// Find position for points with arbitrary # of dimensions
pub fn cubic_bezier<V: Animatable<C>, C: en::Num>(b0: &V, b1: &V, b2: &V, b3: &V, t: f64) -> V {
    let it = 1.0 - t;
    let t0 = b0.map(|v0| en::cast::<C, _>(cube(it)) * v0);
    let t1 = b1.map(|v1| en::cast::<C, _>(3.0 * square(it) * t) * v1);
    let t2 = b2.map(|v2| en::cast::<C, _>(3.0 * it * square(t)) * v2);
    let t3 = b3.map(|v3| en::cast::<C, _>(cube(t)) * v3);

    let result = t0
        .zip_map(t1, |v, v1| v + v1)
        .zip_map(t2, |v, v2| v + v2)
        .zip_map(t3, |v, v3| v + v3);

    result
}

// Find (exact) tangent/velocity for points with arbitrary # of dimensions
pub fn dt_cubic_bezier<V: Animatable<C>, C: en::Num>(b0: &V, b1: &V, b2: &V, b3: &V, t: f64) -> V {
    let it = 1.0 - t;
    let t0 = b0.map(|v0| en::cast::<C, _>(-3.0 * square(it)) * v0);
    let t1 = b1.map(|v1| en::cast::<C, _>(3.0 * it * (it - 2.0 * t)) * v1);
    let t2 = b2.map(|v2| en::cast::<C, _>(3.0 * t * (2.0 * it - t)) * v2);
    let t3 = b3.map(|v3| en::cast::<C, _>(3.0 * square(t)) * v3);

    let result = t0
        .zip_map(t1, |v, v1| v + v1)
        .zip_map(t2, |v, v2| v + v2)
        .zip_map(t3, |v, v3| v + v3);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tolerance for testing numerical accuracy
    const TEST_TOLERANCE_EXACT: f64 = 1e-7;
    const TEST_TOLERANCE_APPROX: f64 = 1e-3;

    // Step size for epsilon/delta slope test
    const TEST_SLOPE_DELTA: f64 = 1e-6;
    const TEST_SLOPE_EPSILON: f64 = 1e-4;

    // Number of steps along curve to test
    const TEST_STEPS: usize = 1000;

    pub fn approx_eq(lhs: f64, rhs: f64, epsilon: f64) -> bool {
        lhs.is_finite() && rhs.is_finite() && ((lhs - epsilon)..(lhs + epsilon)).contains(&rhs)
    }

    #[test]
    fn test() {
        // Three different parametrizations of a diagonal
        test_curve(0.166, 0.166, 0.833, 0.833);
        test_curve(0.333, 0.333, 0.666, 0.666);
        test_curve(0.1, 0.1, 0.666, 0.666);

        // Slight curve
        test_curve(0.125, 0.166, 0.833, 0.833);

        // Ease-out
        test_curve(0.166, 0.0, 0.833, 0.833);

        // Ease-in
        test_curve(0.166, 0.166, 0.833, 1.0);

        // Ease-in-out
        test_curve(0.125, 0.0, 0.833, 1.0);

        // Strong ease-in-out
        test_curve(0.333, 0.0, 0.666, 1.0);

        // Overshoot/undershoot
        test_curve(0.166, -0.25, 0.833, 1.25);
    }

    fn test_curve(ox: f64, oy: f64, ix: f64, iy: f64) {
        test_invert(ox, ix);
        test_smooth(ox, oy, ix, iy);
        test_slope(ox, oy, ix, iy);
    }

    // Test if x(x^-1(t)) is within tolerance everywhere
    fn test_invert(ox: f64, ix: f64) {
        let step: f64 = 1.0 / (TEST_STEPS as f64);
        for i in 0..=TEST_STEPS {
            let ti = (i as f64) * step;

            let to = invert_fixed_bezier(ox, ix, ti);
            let tti = fixed_bezier(ox, ix, to);

            assert!(
                approx_eq(tti, ti, TEST_TOLERANCE_EXACT),
                "identity failed on 1D cubic bezier {} {} at {} != {}",
                ox,
                ix,
                ti,
                tti
            )
        }
    }

    // Test curve smoothness
    // Check if each point is near the average of its neighbors
    fn test_smooth(ox: f64, oy: f64, ix: f64, iy: f64) {
        let step: f64 = 1.0 / (TEST_STEPS as f64);

        for i in 0..=TEST_STEPS {
            let ti = (i as f64) * step;

            let to1 = cubic_bezier_ease(ox, oy, ix, iy, ti - step);
            let to2 = cubic_bezier_ease(ox, oy, ix, iy, ti);
            let to3 = cubic_bezier_ease(ox, oy, ix, iy, ti + step);

            let mid = (to1 + to3) / 2.0;

            assert!(
                approx_eq(mid, to2, TEST_TOLERANCE_APPROX),
                "curve is not smooth at {}: {} {} {}",
                ti,
                to1,
                to2,
                to3
            );
        }
    }

    // Check if start/end slopes match the 2D bezier tangents
    fn test_slope(ox: f64, oy: f64, ix: f64, iy: f64) {
        // Use central difference around start and end
        let t_out2 = cubic_bezier_ease(ox, oy, ix, iy, TEST_SLOPE_DELTA);
        let t_out1 = cubic_bezier_ease(ox, oy, ix, iy, -TEST_SLOPE_DELTA);

        let t_in2 = cubic_bezier_ease(ox, oy, ix, iy, 1.0 + TEST_SLOPE_DELTA);
        let t_in1 = cubic_bezier_ease(ox, oy, ix, iy, 1.0 - TEST_SLOPE_DELTA);

        // Get slopes
        let slope_out = (t_out2 - t_out1) / (2.0 * TEST_SLOPE_DELTA);
        let slope_in = (t_in2 - t_in1) / (2.0 * TEST_SLOPE_DELTA);

        assert!(
            approx_eq(slope_out, oy / ox, TEST_SLOPE_EPSILON),
            "out slope doesn't match {} {}:  {} != {}",
            ox,
            oy,
            oy / ox,
            slope_out,
        );

        assert!(
            approx_eq(slope_in, (1.0 - iy) / (1.0 - ix), TEST_SLOPE_EPSILON),
            "in slope doesn't match {} {}: {} != {}",
            ix,
            iy,
            (1.0 - iy) / (1.0 - ix),
            slope_in,
        );
    }
}
