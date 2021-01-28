use gee::en::{self};
use time_point::Duration;

use crate::coordinate::Coordinate;

const UNIFORM_ALPHA: f64 = 0.0;
const CENTRIPETAL_ALPHA: f64 = 0.5;
const CHORDAL_ALPHA: f64 = 1.0;

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

    let a1 = (*p0 * (d1t / d10)) + (*p1 * (dt0 / d10));
    let a2 = (*p1 * (d2t / d21)) + (*p2 * (-d1t / d21));
    let a3 = (*p2 * (d3t / d32)) + (*p3 * (-d2t / d32));

    let b1 = (a1 * (d2t / d20)) + (a2 * (dt0 / d20));
    let b2 = (a2 * (d3t / d31)) + (a3 * (-d1t / d31));

    let c = (b1 * (d2t / d21)) + (b2 * (-d1t / d21));

    c
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

pub fn centripetal_catmull_rom<X: en::Num, Y: en::Num>(
    p0: Coordinate<X, Y>,
    p1: Coordinate<X, Y>,
    p2: Coordinate<X, Y>,
    p3: Coordinate<X, Y>,
    t: f64,
) -> Coordinate<X, Y> {
    let _t = t_values(&p0, &p1, &p2, &p3, CENTRIPETAL_ALPHA);
    // Our input t value ranges from 0-1, and needs to be scaled to match the spline's knots
    let adjusted_t = _t.1 + ((_t.2 - _t.1) * t);
    catmull_rom_value(&p0, &p1, &p2, &p3, _t.0, _t.1, _t.2, _t.3, adjusted_t)
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
