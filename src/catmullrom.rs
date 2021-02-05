use gee::en::{self};
use time_point::Duration;

use crate::coordinate::Coordinate;

const UNIFORM_ALPHA: f64 = 0.0;
const CENTRIPETAL_ALPHA: f64 = 0.5;
const CHORDAL_ALPHA: f64 = 1.0;

const TANGENT_EPSILON: f64 = 1e-5;

pub fn catmull_rom_value<X: en::Num, Y: en::Num>(
    p0: &Coordinate<X, Y>,
    p1: &Coordinate<X, Y>,
    p2: &Coordinate<X, Y>,
    p3: &Coordinate<X, Y>,
    t0: f64,
    t1: f64,
    t2: f64,
    t3: f64,
    t: f64,
) -> Coordinate<X, Y> {
    // In a Catmull-Rom (CR) spline, four control points are used along with four
    // knots describing the arc lengths between the points. For a centripetal CR
    // spline, the knots (t0-3) are described as follows:
    //
    // t0 = 0
    // ti+1 = distance(pi, pi+1)^alpha + ti
    //
    // The t values may also be arbitrarily spaced (t0 is always 0), and the spline
    // will still be continuous, though the lengths of the arc and the tangents
    // at the control points will be affected.
    //
    // From our t values, we calculate a cubic function describing the arc between
    // points p1 and p2
    //
    // C = (t2 - t / t2 - t1) * B1 + (t - t1 / t2 - t1) * B2
    // B1 = (t2 - t / t2 - t0) * A1 + (t - t0 / t2 - t0) * A2
    // B2 = (t3 - t / t3 - t1) * A2 + (t - t1 / t3 - t1) * A3
    // A1 = (t1 - t / t1 - t0) * P0 + (t - t0 / t1 - t0) * P1
    // A2 = (t2 - t / t2 - t1) * P1 + (t - t1 / t2 - t1) * P2
    // A3 = (t3 - t / t3 - t2) * P3 + (t - t2 / t3 - t2) * P4
    //
    // This cubic function gives the output of the spline for values of t ranging
    // between t1 and t2.

    // The _D_ifference between t_#_ and t_#_
    let d10 = t1 - t0;
    let d1t = t1 - t;
    let d20 = t2 - t0;
    let d21 = t2 - t1;
    let d2t = t2 - t;
    let d31 = t3 - t1;
    let d32 = t3 - t2;
    let d3t = t3 - t;
    let dt0 = t - t0;

    let a1 = if d10 != 0.0 {
        (*p0 * (d1t / d10)) + (*p1 * (dt0 / d10))
    } else {
        *p0
    };
    let a2 = if d21 != 0.0 {
        (*p1 * (d2t / d21)) + (*p2 * (-d1t / d21))
    } else {
        *p1
    };
    let a3 = if d32 != 0.0 {
        (*p2 * (d3t / d32)) + (*p3 * (-d2t / d32))
    } else {
        *p2
    };

    let b1 = if d20 != 0.0 {
        (a1 * (d2t / d20)) + (a2 * (dt0 / d20))
    } else {
        a1
    };
    let b2 = if d31 != 0.0 {
        (a2 * (d3t / d31)) + (a3 * (-d1t / d31))
    } else {
        a2
    };

    let c = if d21 != 0.0 {
        (b1 * (d2t / d21)) + (b2 * (-d1t / d21))
    } else {
        b1
    };

    c
}

// Convert non-uniform catmull rom to equivalent bezier spline
//
// Uses numerical approximation
pub fn catmull_rom_to_bezier<X: en::Num, Y: en::Num>(
    p0: &Coordinate<X, Y>,
    p1: &Coordinate<X, Y>,
    p2: &Coordinate<X, Y>,
    p3: &Coordinate<X, Y>,
    t0: f64,
    t1: f64,
    t2: f64,
    t3: f64,
) -> (
    Coordinate<X, Y>,
    Coordinate<X, Y>,
    Coordinate<X, Y>,
    Coordinate<X, Y>,
) {
    // Inner knot distance
    let s = t2 - t1;

    // Sample central difference around start and end
    let a1 = catmull_rom_value(&p0, &p1, &p2, &p3, t0, t1, t2, t3, t1 - s * TANGENT_EPSILON);
    let b1 = catmull_rom_value(&p0, &p1, &p2, &p3, t0, t1, t2, t3, t1 + s * TANGENT_EPSILON);

    let a2 = catmull_rom_value(&p0, &p1, &p2, &p3, t0, t1, t2, t3, t2 - s * TANGENT_EPSILON);
    let b2 = catmull_rom_value(&p0, &p1, &p2, &p3, t0, t1, t2, t3, t2 + s * TANGENT_EPSILON);

    // Scale to appropriate range
    // Bezier has factor of 3, central difference has factor of 2
    let d1 = (b1 - a1) * (1.0 / (TANGENT_EPSILON * 6.0));
    let d2 = (a2 - b2) * (1.0 / (TANGENT_EPSILON * 6.0));

    (*p1, *p1 + d1, *p2 + d2, *p2)
}

// Cubic bezier with endpoints p0 / p3 and control points p1 / 2
pub fn bezier_value<X: en::Num, Y: en::Num>(
    p0: &Coordinate<X, Y>,
    p1: &Coordinate<X, Y>,
    p2: &Coordinate<X, Y>,
    p3: &Coordinate<X, Y>,
    t: f64,
) -> Coordinate<X, Y> {
    let t2 = t * t;
    let t3 = t * t2;

    let it = 1.0 - t;
    let it2 = it * it;
    let it3 = it * it2;

    *p0 * it3 + *p1 * it2 * (3.0 * t) + *p2 * it * (3.0 * t2) + *p3 * t3
}

// Calculate values of T for a given alpha
// alpha = 0.0: Uniform spline
// alpha = 0.5: Centripetal spline
// alpha = 1.0: Chordal spline
pub fn t_values<X: en::Num, Y: en::Num>(
    p0: &Coordinate<X, Y>,
    p1: &Coordinate<X, Y>,
    p2: &Coordinate<X, Y>,
    p3: &Coordinate<X, Y>,
    alpha: f64,
) -> (f64, f64, f64, f64) {
    let t1 = f64::powf(p0.distance_to(&p1), alpha);
    let t2 = f64::powf(p1.distance_to(&p2), alpha) + t1;
    let t3 = f64::powf(p2.distance_to(&p3), alpha) + t2;

    (0.0, t1, t2, t3)
}

// Calculate values of T, ignoring Y dimension
pub fn t_values_x<X: en::Num, Y: en::Num>(
    p0: &Coordinate<X, Y>,
    p1: &Coordinate<X, Y>,
    p2: &Coordinate<X, Y>,
    p3: &Coordinate<X, Y>,
    alpha: f64,
) -> (f64, f64, f64, f64) {
    let t1 = f64::powf(en::cast::<f64, _>(p1.x - p0.x).abs(), alpha);
    let t2 = f64::powf(en::cast::<f64, _>(p2.x - p1.x).abs(), alpha) + t1;
    let t3 = f64::powf(en::cast::<f64, _>(p3.x - p2.x).abs(), alpha) + t2;

    (0.0, t1, t2, t3)
}

// Calculate values of T, ignoring X dimension
pub fn t_values_y<X: en::Num, Y: en::Num>(
    p0: &Coordinate<X, Y>,
    p1: &Coordinate<X, Y>,
    p2: &Coordinate<X, Y>,
    p3: &Coordinate<X, Y>,
    alpha: f64,
) -> (f64, f64, f64, f64) {
    let t1 = f64::powf(en::cast::<f64, _>(p1.y - p0.y).abs(), alpha);
    let t2 = f64::powf(en::cast::<f64, _>(p2.y - p1.y).abs(), alpha) + t1;
    let t3 = f64::powf(en::cast::<f64, _>(p3.y - p2.y).abs(), alpha) + t2;

    (0.0, t1, t2, t3)
}

pub fn centripetal_catmull_rom<X: en::Num, Y: en::Num>(
    p0: Coordinate<X, Y>,
    p1: Coordinate<X, Y>,
    p2: Coordinate<X, Y>,
    p3: Coordinate<X, Y>,
    t: f64,
) -> Coordinate<X, Y> {
    let (t0, t1, t2, t3) = t_values(&p0, &p1, &p2, &p3, CENTRIPETAL_ALPHA);

    // Our input t value ranges from 0-1, and needs to be scaled to match the spline's knots
    let adjusted_t = t1 + ((t2 - t1) * t);
    catmull_rom_value(&p0, &p1, &p2, &p3, t0, t1, t2, t3, adjusted_t)
}

pub fn catmull_rom_time_scale(
    d0: Duration,
    d1: Duration,
    d2: Duration,
    d3: Duration,
    elapsed: Duration,
) -> Duration {
    // A curve which goes through durations d0-3 (x) at equidistant y positions allows us to
    // transform elapsed time into elapsed spline time
    let t_value = if d0 == d1 {
        elapsed.nanos as f64 / d2.nanos as f64
    } else {
        (elapsed.nanos - d1.nanos) as f64 / (d2.nanos - d1.nanos) as f64
    };

    // Y-axis represents elapsed time as a percentage, 0-1
    let relative_elapsed_spline_time = centripetal_catmull_rom(
        Coordinate::new(d0.nanos, 0.0),
        Coordinate::new(d1.nanos, 0.3333),
        Coordinate::new(d2.nanos, 0.6666),
        Coordinate::new(d3.nanos, 1.0),
        t_value,
    )
    .y;

    Duration::new((relative_elapsed_spline_time * d3.nanos as f64) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Accuracy threshold for matching curves
    const TEST_EPSILON: f64 = 1e-7;

    // Steps to divide curve into
    const TEST_STEPS: usize = 1000;

    #[test]
    fn test_match_cr_bez() {
        let p1 = Coordinate::new(0.0, 0.0);
        let p2 = Coordinate::new(1.0, 0.0);
        let p3 = Coordinate::new(2.0, 2.1);
        let p4 = Coordinate::new(-1.0, 4.0);

        let t1: f64 = -0.04;
        let t2: f64 = 0.15;
        let t3: f64 = 0.2;
        let t4: f64 = 0.3;

        let bezier = catmull_rom_to_bezier(&p1, &p2, &p3, &p4, t1, t2, t3, t4);
        let b1 = bezier.0;
        let b2 = bezier.1;
        let b3 = bezier.2;
        let b4 = bezier.3;

        for i in 0..=TEST_STEPS {
            let d = (i as f64) / (TEST_STEPS as f64);
            let cr = catmull_rom_value(&p1, &p2, &p3, &p4, t1, t2, t3, t4, t2 + (t3 - t2) * d);
            let bz = bezier_value(&b1, &b2, &b3, &b4, d);

            assert!(
                cr.distance_to(&bz) < TEST_EPSILON,
                "bezier does not match catmull rom at {}: {},{} != {},{}",
                d,
                cr.x,
                cr.y,
                bz.x,
                bz.y
            );
        }
    }

    #[test]
    fn test_degen_knots() {
        let p1 = Coordinate::new(0.0, 0.0);
        let p2 = p1;
        let p3 = Coordinate::new(2.0, 0.0);
        let p4 = Coordinate::new(-1.0, 4.0);

        let t1: f64 = -0.1;
        let t2: f64 = t1;
        let t3: f64 = 0.2;
        let t4: f64 = 0.3;

        let bezier = catmull_rom_to_bezier(&p1, &p2, &p3, &p4, t1, t2, t3, t4);
        let b1 = bezier.0;
        let b2 = bezier.1;
        let b3 = bezier.2;
        let b4 = bezier.3;

        for i in 0..=TEST_STEPS {
            let d = (i as f64) / (TEST_STEPS as f64);
            let cr = catmull_rom_value(&p1, &p2, &p3, &p4, t1, t2, t3, t4, t2 + (t3 - t2) * d);
            let bz = bezier_value(&b1, &b2, &b3, &b4, d);

            assert!(
                cr.distance_to(&bz) < TEST_EPSILON,
                "bezier does not match catmull rom at {}: {},{} != {},{}",
                d,
                cr.x,
                cr.y,
                bz.x,
                bz.y
            );
        }
    }
}
