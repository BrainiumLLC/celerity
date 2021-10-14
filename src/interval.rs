use crate::{
    spline::{
        bezier::{cubic_bezier, cubic_bezier_ease, fixed_bezier},
        bezier_ease::BezierEase,
        bezier_path::BezierPath,
        spline_ease, SplineMap,
    },
    Animatable, Animation, BoundedAnimation,
};
use std::time::Duration;

// A half-interval
#[derive(Copy, Clone, Debug)]
pub struct Frame<V: Animatable> {
    pub offset: Duration,
    pub value: V,
}

impl<V: Animatable> Frame<V> {
    pub fn new(offset: Duration, value: V) -> Self {
        Self { offset, value }
    }
}

#[derive(Clone, Debug)]
pub struct Interval<V: Animatable> {
    pub start: Duration,
    pub end: Duration,
    pub from: V,
    pub to: V,
    pub ease: Option<BezierEase>,
    pub path: Option<BezierPath<V>>,
    pub reticulated_spline: Option<SplineMap>,
}

#[derive(Clone, Debug)]
pub struct InspectInterval<V: Animatable> {
    pub start: Duration,
    pub end: Duration,
    pub ease: Vec<(f64, f64)>,
    pub path: Vec<V>,
    pub reticulated_spline: Option<Vec<(f64, f64)>>,
}

impl<V: Animatable> Interval<V> {
    pub fn new(
        start: Duration,
        end: Duration,
        from: V,
        to: V,
        ease: Option<BezierEase>,
        path: Option<BezierPath<V>>,
        reticulated_spline: Option<SplineMap>,
    ) -> Self {
        Self {
            start,
            end,
            from,
            to,
            ease,
            path,
            reticulated_spline,
        }
    }

    pub fn eased(a: Frame<V>, b: Frame<V>, ox: f64, oy: f64, ix: f64, iy: f64) -> Self {
        Self::new(
            a.offset,
            b.offset,
            a.value,
            b.value,
            Some(BezierEase::new(ox, oy, ix, iy)),
            None,
            None,
        )
    }

    pub fn transition(a: Frame<V>, b: Frame<V>, ease: Option<BezierEase>) -> Self {
        Self::new(a.offset, b.offset, a.value, b.value, ease, None, None)
    }

    pub fn linear(a: Frame<V>, b: Frame<V>) -> Self {
        Self::new(a.offset, b.offset, a.value, b.value, None, None, None)
    }

    pub fn hold(value: V, duration: Duration) -> Self {
        Self::new(Duration::ZERO, duration, value, value, None, None, None)
    }

    pub fn from_values(duration: Duration, from: V, to: V, ease: Option<BezierEase>) -> Self {
        Self::new(Duration::ZERO, duration, from, to, ease, None, None)
    }

    pub fn percent_elapsed(&self, elapsed: Duration) -> f64 {
        if self.duration().is_zero() {
            0.0
        } else {
            (elapsed.clamp(self.start, self.end) - self.start).as_secs_f64()
                / self.duration().as_secs_f64()
        }
    }

    pub fn length(&self) -> f64 {
        if let Some(splinemap) = &self.reticulated_spline {
            splinemap.length
        } else {
            self.from.distance_to(self.to)
        }
    }

    pub fn average_speed(&self) -> f64 {
        self.length() / self.duration().as_secs_f64()
    }

    #[allow(dead_code)]
    pub fn inspect(&self, detail: usize) -> InspectInterval<V> {
        let sample_ease = |ease: &BezierEase| {
            (0..detail)
                .map(|i| {
                    let t = (i as f64) / (detail as f64);
                    (
                        fixed_bezier(ease.ox, ease.ix, t),
                        fixed_bezier(ease.oy, ease.iy, t),
                    )
                })
                .collect()
        };

        InspectInterval {
            start: self.start,
            end: self.end,
            path: self.path(detail, self.end - self.start),
            ease: match &self.ease {
                Some(ease) => sample_ease(ease),
                None => vec![(0.0, 0.0), (1.0, 1.0)],
            },
            reticulated_spline: self
                .reticulated_spline
                .as_ref()
                .map(|reticulated_spline| reticulated_spline.steps.to_vec()),
        }
    }
}

impl<V: Animatable> Animation<V> for Interval<V> {
    fn sample(&self, elapsed: Duration) -> V {
        // Apply temporal easing (or not)
        let percent_elapsed = self.percent_elapsed(elapsed);
        let eased_time = self
            .ease
            .as_ref()
            .map(|e| cubic_bezier_ease(e.ox, e.oy, e.ix, e.iy, percent_elapsed))
            .unwrap_or(percent_elapsed);

        // Map eased distance to spline time using spline map (or not)
        let spline_time = self
            .reticulated_spline
            .as_ref()
            .map(|m| spline_ease(&m, eased_time))
            .unwrap_or(eased_time);

        // Look up value along spline (or lerp)
        let value = self
            .path
            .as_ref()
            .map(|p| cubic_bezier(&self.from, &p.b1, &p.b2, &self.to, spline_time))
            .unwrap_or_else(|| self.from.lerp(self.to, spline_time));
        value
    }
}

impl<V: Animatable> BoundedAnimation<V> for Interval<V> {
    fn duration(&self) -> Duration {
        self.end - self.start
    }
}
