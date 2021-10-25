use super::bezier::cubic_bezier_ease;
use gee::Point;

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
    pub const fn new(ox: f64, oy: f64, ix: f64, iy: f64) -> Self {
        Self { ox, oy, ix, iy }
    }

    pub fn as_points(&self) -> (Point<f64>, Point<f64>, Point<f64>, Point<f64>) {
        let b0 = Point::new(0.0, 0.0);
        let b1 = Point::new(self.ox, self.oy);
        let b2 = Point::new(self.ix, self.iy);
        let b3 = Point::new(1.0, 1.0);

        (b0, b1, b2, b3)
    }

    pub const fn linear() -> Self {
        Self::new(0.16, 0.16, 0.84, 0.84)
    }
    pub const fn ease_in() -> Self {
        Self::new(0.16, 0.0, 0.84, 0.84)
    }
    pub const fn ease_out() -> Self {
        Self::new(0.16, 0.16, 0.84, 1.0)
    }
    pub const fn ease_in_out() -> Self {
        Self::new(0.16, 0.0, 0.84, 1.0)
    }

    pub fn ease(&self, t: f64) -> f64 {
        cubic_bezier_ease(self.ox, self.oy, self.ix, self.iy, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        interval::Interval, interval_track::IntervalTrack, spline::bezier_path::BezierPath,
        spline::SplineMap, Animatable, Animation as _,
    };
    use gee::en::Num as _;
    use std::time::Duration;

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
            reticulated_spline: None,
        };

        let track = IntervalTrack::new().with_interval(interval);

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
            reticulated_spline: None,
        };

        let track = IntervalTrack::new().with_interval(interval);

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
            reticulated_spline: None,
        };

        let track = IntervalTrack::new().with_interval(interval);

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
        let spline_map = SplineMap::from_bezier(&from, &b1, &b2, &to, true);

        let length = spline_map.length;
        println!("length {}", length);

        let interval = Interval {
            start,
            end,
            from,
            to,
            ease: None,
            path: Some(BezierPath { b1, b2 }),
            reticulated_spline: Some(spline_map),
        };

        let track = IntervalTrack::new().with_interval(interval);

        assert!(approx_eq_point(track.sample(start), from, TOLERANCE));
        assert!(approx_eq_point(track.sample(end), to, TOLERANCE));

        // Animation should be linear by arc length
        let steps: usize = 100;
        let step = length / steps.to_f64();
        for i in 0..steps {
            let t1 = i.to_f64() / steps.to_f64();
            let t2 = (i + 1).to_f64() / steps.to_f64();
            let p1 = track.sample(start + (end - start).mul_f64(t1));
            let p2 = track.sample(start + (end - start).mul_f64(t2));
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
        let spline_map = SplineMap::from_bezier(&from, &b1, &b2, &to, true);

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
            path: Some(BezierPath { b1, b2 }),
            reticulated_spline: Some(spline_map),
        };

        let track = IntervalTrack::new().with_interval(interval);

        assert!(approx_eq_point(track.sample(start), from, TOLERANCE));
        assert!(approx_eq_point(track.sample(end), to, TOLERANCE));

        // Animation should be eased perfectly symmetricly, on a curved path
        let steps: usize = 100;
        let step = length / (steps as f64);
        for i in 0..steps {
            let t1 = i.to_f64() / steps.to_f64();
            let t2 = (i + 1).to_f64() / steps.to_f64();

            let t3 = 1.0 - i.to_f64() / steps.to_f64();
            let t4 = 1.0 - (i + 1).to_f64() / steps.to_f64();

            let p1 = track.sample(start + (end - start).mul_f64(t1));
            let p2 = track.sample(start + (end - start).mul_f64(t2));

            let p3 = track.sample(start + (end - start).mul_f64(t3));
            let p4 = track.sample(start + (end - start).mul_f64(t4));

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
