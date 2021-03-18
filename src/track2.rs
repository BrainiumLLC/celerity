use std::marker::PhantomData;

use crate::{
    ease::{
        bezier::{cubic_bezier, cubic_bezier_ease},
        spline::{spline_ease, SplineMap},
    },
    Animatable, Animation, BoundedAnimation,
};
use gee::en;
use time_point::Duration;

#[derive(Clone, Debug)]
pub struct IntervalTrack<V: Animatable<C>, C: en::Num> {
    intervals: Vec<Interval<V, C>>,
}

#[derive(Clone, Debug)]
pub struct Interval<V: Animatable<C>, C: en::Num> {
    pub start: Duration,
    pub end: Duration,
    pub from: V,
    pub to: V,
    pub ease: Option<BezierEase>,
    pub path: Option<BezierPath<V, C>>,
    pub metric: Option<SplineMap>,
    _marker: PhantomData<C>,
}

// Describes the temporal Bezier ease between two Animatables
// as a relative curve from (0, 0) to (1, 1).
//
// X values always range [0...1]
// Y values usually range [0...1]
#[derive(Clone, Debug)]
pub struct BezierEase {
    pub ox: f64,
    pub oy: f64,
    pub ix: f64,
    pub iy: f64,
}

impl BezierEase {
    pub fn new(ox: f64, oy: f64, ix: f64, iy: f64) -> Self {
        Self { ox, oy, ix, iy }
    }
}

// Describes the two middle control points for a bezier path
// between an interval's spatial endpoints.
//
// These are in absolute coordinates,
// i.e. (from, b1, b2, to) is a bezier.
#[derive(Clone, Debug)]
pub struct BezierPath<V: Animatable<C>, C: en::Num> {
    pub b1: V,
    pub b2: V,
    _marker: PhantomData<C>,
}

impl<V: Animatable<C>, C: en::Num> BezierPath<V, C> {
    pub fn new(b1: V, b2: V) -> Self {
        Self {
            b1,
            b2,
            _marker: PhantomData,
        }
    }
}

impl<V: Animatable<C>, C: en::Num> IntervalTrack<V, C> {
    pub fn current_interval(&self, elapsed: &Duration) -> Option<&Interval<V, C>> {
        self.intervals
            .iter()
            .find(|interval| interval.end < *elapsed)
            .or_else(|| self.intervals.last())
    }
}

impl<V: Animatable<C>, C: en::Num> Animation<V, C> for IntervalTrack<V, C> {
    fn sample(&self, elapsed: Duration) -> V {
        // Get interval
        let interval = self.current_interval(&elapsed).unwrap();

        // Clamp time to current interval
        let time = elapsed.clamp(interval.start, interval.end);

        // Apply temporal easing (or not)
        let fraction = (time - interval.start).div_duration_f64(interval.end - interval.start);
        let eased_time = interval
            .ease
            .as_ref()
            .map(|e| cubic_bezier_ease(e.ox, e.oy, e.ix, e.iy, fraction))
            .unwrap_or(fraction);

        // Map eased distance to spline time using spline map (or not)
        let spline_time = interval
            .metric
            .as_ref()
            .map(|m| spline_ease(&m, eased_time))
            .unwrap_or(eased_time);

        // Look up value along spline (or lerp)
        let value = interval
            .path
            .as_ref()
            .map(|p| cubic_bezier(&interval.from, &p.b1, &p.b2, &interval.to, spline_time))
            .unwrap_or_else(|| interval.from.lerp(interval.to, spline_time));
        value
    }
}

impl<V: Animatable<C>, C: en::Num> BoundedAnimation<V, C> for IntervalTrack<V, C> {
    fn duration(&self) -> Duration {
        self.intervals
            .last()
            .map(|interval| interval.end)
            .unwrap_or(Duration::from_secs_f64(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-4;
    const TOLERANCE_LOOSE: f64 = 1e-3;

    pub fn approx_eq(lhs: f64, rhs: f64, epsilon: f64) -> bool {
        let eq =
            lhs.is_finite() && rhs.is_finite() && ((lhs - epsilon)..(lhs + epsilon)).contains(&rhs);
        if !eq {
            println!("{} != {}", lhs, rhs);
        }
        eq
    }

    pub fn approx_eq_point(lhs: gee::Point<f64>, rhs: gee::Point<f64>, epsilon: f64) -> bool {
        approx_eq(lhs.x, rhs.x, epsilon) && approx_eq(lhs.y, rhs.y, epsilon)
    }

    #[test]
    fn test_scalar_linear() {
        let start = Duration::from_secs_f64(10.0);
        let q1 = Duration::from_secs_f64(12.5);
        let mid = Duration::from_secs_f64(15.0);
        let q2 = Duration::from_secs_f64(17.5);
        let end = Duration::from_secs_f64(20.0);

        let from: f64 = 1.0;
        let to: f64 = 3.0;

        let interval = Interval {
            start,
            end,
            from,
            to,
            ease: None,
            path: None,
            metric: None,
            _marker: PhantomData,
        };

        let track = IntervalTrack {
            intervals: vec![interval],
        };

        // Animation should be linear
        assert!(approx_eq(track.sample(start), from, TOLERANCE));
        assert!(approx_eq(track.sample(end), to, TOLERANCE));
        assert!(approx_eq(track.sample(mid), from.lerp(to, 0.5), TOLERANCE));
        assert!(approx_eq(track.sample(q1), from.lerp(to, 0.25), TOLERANCE));
        assert!(approx_eq(track.sample(q2), from.lerp(to, 0.75), TOLERANCE));
    }

    #[test]
    fn test_scalar_eased() {
        let start = Duration::from_secs_f64(10.0);
        let q1 = Duration::from_secs_f64(12.5);
        let mid = Duration::from_secs_f64(15.0);
        let q2 = Duration::from_secs_f64(17.5);
        let end = Duration::from_secs_f64(20.0);

        let from: f64 = 1.0;
        let to: f64 = 3.0;

        let interval = Interval {
            start,
            end,
            from,
            to,
            ease: Some(BezierEase {
                ox: 0.5,
                oy: 0.0,
                ix: 0.5,
                iy: 1.0,
            }),
            path: None,
            metric: None,
            _marker: PhantomData,
        };

        let track = IntervalTrack {
            intervals: vec![interval],
        };

        // Animation should ease towards start and end
        assert!(approx_eq(track.sample(start), from, TOLERANCE));
        assert!(approx_eq(track.sample(end), to, TOLERANCE));
        assert!(approx_eq(track.sample(mid), from.lerp(to, 0.5), TOLERANCE));
        assert!(approx_eq(
            track.sample(q1),
            from.lerp(to, 0.1059),
            TOLERANCE
        ));
        assert!(approx_eq(
            track.sample(q2),
            from.lerp(to, 0.8941),
            TOLERANCE
        ));
    }

    #[test]
    fn test_vector_eased() {
        let start = Duration::from_secs_f64(10.0);
        let q1 = Duration::from_secs_f64(12.5);
        let mid = Duration::from_secs_f64(15.0);
        let q2 = Duration::from_secs_f64(17.5);
        let end = Duration::from_secs_f64(20.0);

        let from: gee::Point<f64> = gee::Point::new(0.0, 0.0);
        let to: gee::Point<f64> = gee::Point::new(3.0, 4.0);

        let interval = Interval {
            start,
            end,
            from,
            to,
            ease: Some(BezierEase {
                ox: 0.5,
                oy: 0.0,
                ix: 0.5,
                iy: 1.0,
            }),
            path: None,
            metric: None,
            _marker: PhantomData,
        };

        let track = IntervalTrack {
            intervals: vec![interval],
        };

        // Animation should ease towards start and end
        assert!(approx_eq_point(track.sample(start), from, TOLERANCE));
        assert!(approx_eq_point(track.sample(end), to, TOLERANCE));
        assert!(approx_eq_point(
            track.sample(mid),
            from.lerp(to, 0.5),
            TOLERANCE
        ));
        assert!(approx_eq_point(
            track.sample(q1),
            from.lerp(to, 0.1059),
            TOLERANCE
        ));
        assert!(approx_eq_point(
            track.sample(q2),
            from.lerp(to, 0.8941),
            TOLERANCE
        ));
    }

    #[test]
    fn test_vector_linear_path() {
        let start = Duration::from_secs_f64(10.0);
        let end = Duration::from_secs_f64(20.0);

        let from: gee::Point<f64> = gee::Point::new(-4.0, 0.0);
        let to: gee::Point<f64> = gee::Point::new(4.0, 0.0);

        let b1: gee::Point<f64> = gee::Point::new(-4.0, -4.0);
        let b2: gee::Point<f64> = gee::Point::new(4.0, 4.0);

        //let spline_map = SplineMap::from_spline(|t| cubic_bezier(&from, &b1, &b2, &to, t));
        let spline_map = SplineMap::from_bezier(&from, &b1, &b2, &to);

        let length = spline_map.length;
        println!("length {}", length);

        let interval = Interval {
            start,
            end,
            from,
            to,
            ease: None,
            path: Some(BezierPath {
                b1,
                b2,
                _marker: PhantomData,
            }),
            metric: Some(spline_map),
            _marker: PhantomData,
        };

        let track = IntervalTrack {
            intervals: vec![interval],
        };

        assert!(approx_eq_point(track.sample(start), from, TOLERANCE));
        assert!(approx_eq_point(track.sample(end), to, TOLERANCE));

        // Animation should be linear by arc length
        let steps: usize = 100;
        let step = length / (steps as f64);
        for i in 0..steps {
            let t1 = en::cast::<f64, _>(i) / en::cast::<f64, _>(steps);
            let t2 = en::cast::<f64, _>(i + 1) / en::cast::<f64, _>(steps);
            let p1 = track.sample(start + t1 * (end - start));
            let p2 = track.sample(start + t2 * (end - start));
            println!("{} {} @ {}", p1.distance_to(p2), step, t1);
            assert!(
                approx_eq(p1.distance_to(p2), step, TOLERANCE_LOOSE),
                "unequal step at {}",
                t1
            );
        }
    }

    #[test]
    fn test_vector_eased_path() {
        let start = Duration::from_secs_f64(10.0);
        let end = Duration::from_secs_f64(20.0);

        let from: gee::Point<f64> = gee::Point::new(-4.0, 0.0);
        let to: gee::Point<f64> = gee::Point::new(4.0, 0.0);

        let b1: gee::Point<f64> = gee::Point::new(-4.0, -4.0);
        let b2: gee::Point<f64> = gee::Point::new(4.0, 4.0);

        //let spline_map = SplineMap::from_spline(|t| cubic_bezier(&from, &b1, &b2, &to, t));
        let spline_map = SplineMap::from_bezier(&from, &b1, &b2, &to);

        let length = spline_map.length;
        println!("length {}", length);

        let interval = Interval {
            start,
            end,
            from,
            to,
            ease: Some(BezierEase {
                ox: 0.5,
                oy: 0.0,
                ix: 0.5,
                iy: 1.0,
            }),
            path: Some(BezierPath {
                b1,
                b2,
                _marker: PhantomData,
            }),
            metric: Some(spline_map),
            _marker: PhantomData,
        };

        let track = IntervalTrack {
            intervals: vec![interval],
        };

        assert!(approx_eq_point(track.sample(start), from, TOLERANCE));
        assert!(approx_eq_point(track.sample(end), to, TOLERANCE));

        // Animation should be eased perfectly symmetricly, on a curved path
        let steps: usize = 100;
        let step = length / (steps as f64);
        for i in 0..steps {
            let t1 = en::cast::<f64, _>(i) / en::cast::<f64, _>(steps);
            let t2 = en::cast::<f64, _>(i + 1) / en::cast::<f64, _>(steps);

            let t3 = 1.0 - en::cast::<f64, _>(i) / en::cast::<f64, _>(steps);
            let t4 = 1.0 - en::cast::<f64, _>(i + 1) / en::cast::<f64, _>(steps);

            let p1 = track.sample(start + t1 * (end - start));
            let p2 = track.sample(start + t2 * (end - start));

            let p3 = track.sample(start + t3 * (end - start));
            let p4 = track.sample(start + t4 * (end - start));

            println!("{} {} @ {}", p1.distance_to(p2), step, t1);
            println!("{} {} @ {}", p3.distance_to(p4), step, t4);
            assert!(
                approx_eq(p1.distance_to(p2), p3.distance_to(p4), TOLERANCE_LOOSE),
                "unequal step at {}/{}",
                t1,
                t4
            );
        }
    }
}
