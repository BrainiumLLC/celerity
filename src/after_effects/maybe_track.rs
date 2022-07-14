use super::conv::{FromMultiDimensional, FromValue};
use crate::{
    ease::Ease, interval::Interval, interval_track::IntervalTrack, spline::bezier_ease::BezierEase,
    Animatable, Animation,
};
use bodymovin::properties::{self, ScalarKeyframe, Value};
use std::time::Duration;

impl From<bodymovin::properties::Bezier2d> for BezierEase {
    fn from(bezier: bodymovin::properties::Bezier2d) -> Self {
        Self {
            ox: bezier.out_value.x,
            oy: bezier.out_value.y,
            ix: bezier.in_value.x,
            iy: bezier.in_value.y,
        }
    }
}

impl From<bodymovin::properties::Bezier3d> for BezierEase {
    fn from(bezier: bodymovin::properties::Bezier3d) -> Self {
        Self {
            ox: bezier.out_value.x[0],
            oy: bezier.out_value.y[0],
            ix: bezier.in_value.x[0],
            iy: bezier.in_value.y[0],
        }
    }
}

#[derive(Debug)]
pub enum MaybeTrack<V: Animatable> {
    Fixed(V),
    Animated(IntervalTrack<V>),
}

impl<V: Animatable> Animation<V> for MaybeTrack<V> {
    fn sample(&self, elapsed: Duration) -> V {
        match self {
            Self::Fixed(value) => *value,
            Self::Animated(track) => track.sample(elapsed),
        }
    }
}

#[derive(Debug)]
struct KeyframePair<V> {
    start: Duration,
    end: Duration,
    from: V,
    to: V,
    ease: Option<BezierEase>,
}

impl<V: FromValue> Interval<V> {
    fn from_scalar_keyframes(
        frame_a: &ScalarKeyframe,
        frame_b: &ScalarKeyframe,
        frame_rate: f64,
    ) -> Result<Interval<V>, V::Error> {
        let from_value = V::from_value(
            frame_a
                .start_value
                .expect("ScalarKeyframe is missing start_value.")
                .0,
        )?;
        let to_value = if !frame_a.hold {
            V::from_value(
                frame_a
                    .end_value
                    .unwrap_or_else(|| {
                        frame_b
                            .start_value
                            .expect("ScalarKeyframe is missing start_value.")
                    })
                    .0,
            )?
        } else {
            from_value
        };
        Ok(Interval {
            start: Duration::from_secs_f64(frame_a.start_time / frame_rate),
            end: Duration::from_secs_f64(frame_b.start_time / frame_rate),
            from: from_value,
            to: to_value,
            ease: frame_a.bezier.clone().map(|ease| match ease {
                bodymovin::properties::BezierEase::_2D(ease) => Ease::Bezier(Into::into(ease)),
                bodymovin::properties::BezierEase::_3D(ease) => Ease::Bezier(Into::into(ease)),
            }),
            path: None,
            reticulated_spline: None,
        })
    }
}

impl<V: FromMultiDimensional> Interval<V> {
    pub(crate) fn from_multidimensional_keyframes(
        from: &bodymovin::properties::MultiDimensionalKeyframe,
        to: &bodymovin::properties::MultiDimensionalKeyframe,
        frame_rate: f64,
    ) -> Result<Interval<V>, V::Error> {
        let from_value = V::from_multi_dimensional(
            from.start_value
                .as_ref()
                .expect("Attempted to create Interval with no starting value."),
        )?;
        let to_value = if !from.hold {
            V::from_multi_dimensional(to.start_value.as_ref().unwrap_or(&vec![0.0, 0.0]))?
        } else {
            from_value
        };
        Ok(Interval {
            start: Duration::from_secs_f64(from.start_time.abs() / frame_rate),
            end: Duration::from_secs_f64(to.start_time.abs() / frame_rate),
            from: from_value,
            to: to_value,
            ease: from.bezier.clone().map(|ease| match ease {
                bodymovin::properties::BezierEase::_2D(ease) => Ease::Bezier(Into::into(ease)),
                bodymovin::properties::BezierEase::_3D(ease) => Ease::Bezier(Into::into(ease)),
            }),
            path: None,
            reticulated_spline: None,
        })
    }
}

impl<V: FromValue> MaybeTrack<V> {
    pub(crate) fn from_value(
        value: Value<f64, ScalarKeyframe>,
        frame_rate: f64,
    ) -> Result<Self, V::Error> {
        match value {
            Value::Fixed(value) => V::from_value(value).map(Self::Fixed),
            Value::Animated(keyframes) => keyframes
                .windows(2)
                .map(|window| Interval::from_scalar_keyframes(&window[0], &window[1], frame_rate))
                .collect::<Result<Vec<_>, _>>()
                .map(IntervalTrack::from_intervals)
                .map(Self::Animated),
        }
    }

    pub(crate) fn from_property(
        property: bodymovin::properties::Property<f64, ScalarKeyframe>,
        frame_rate: f64,
    ) -> Result<Self, V::Error> {
        Self::from_value(property.value, frame_rate)
    }
}

impl<V: FromMultiDimensional> MaybeTrack<V> {
    pub(crate) fn from_multi_dimensional(
        multi: properties::MultiDimensional,
        frame_rate: f64,
    ) -> Result<Self, V::Error> {
        match multi.value {
            Value::Fixed(value) => V::from_multi_dimensional(&value).map(Self::Fixed),
            Value::Animated(keyframes) => keyframes
                .windows(2)
                .map(|window| {
                    Interval::from_multidimensional_keyframes(&window[0], &window[1], frame_rate)
                })
                .collect::<Result<Vec<_>, _>>()
                .map(IntervalTrack::from_intervals)
                .map(Self::Animated),
        }
    }
}
