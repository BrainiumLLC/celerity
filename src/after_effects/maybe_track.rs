use super::conv::{FromMultiDimensional, FromValue};
use crate::{
    interval::{BezierEase, Interval, IntervalTrack},
    Animatable, Animation,
};
use bodymovin::properties::{EitherMultiDimensional, EitherValue, MultiDimensional, Value};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

#[derive(Debug)]
pub enum MaybeTrack<V: Animatable<C>, C: en::Num = V> {
    Fixed(V),
    Animated(IntervalTrack<V, C>),
}

impl<V: Animatable<C>, C: en::Num> Animation<V, C> for MaybeTrack<V, C> {
    fn sample(&self, elapsed: Duration) -> V {
        match self {
            Self::Fixed(value) => *value,
            Self::Animated(track) => track.sample(elapsed),
        }
    }
}

#[derive(Debug)]
struct KeyframePair<V, C> {
    start: Duration,
    end: Duration,
    from: V,
    to: V,
    ease: Option<BezierEase>,
    _marker: PhantomData<C>,
}

impl<V: FromValue<C>, C: en::Num> Interval<V, C> {
    fn from_value_keyframes(
        [from, to]: [&bodymovin::properties::ValueKeyframe; 2],
        frame_rate: f64,
    ) -> Interval<V, C> {
        let from_value = V::from_value(from.start_value);
        Interval {
            start: Duration::from_secs_f64(from.start_time / frame_rate),
            end: Duration::from_secs_f64(to.start_time / frame_rate),
            from: from_value,
            to: if !from.hold {
                V::from_value(to.start_value)
            } else {
                from_value
            },
            ease: from.bezier.clone().map(Into::into),
            path: None,
            metric: None,
        }
    }
}

impl<V: FromMultiDimensional<C>, C: en::Num> Interval<V, C> {
    pub(crate) fn from_offset_keyframes(
        [from, to]: [&bodymovin::properties::OffsetKeyframe; 2],
        frame_rate: f64,
    ) -> Interval<V, C> {
        let from_value = V::from_multi_dimensional(&from.start_value).unwrap();
        Interval {
            start: Duration::from_secs_f64(from.start_time / frame_rate),
            end: Duration::from_secs_f64(to.start_time / frame_rate),
            from: from_value,
            to: if !from.hold {
                V::from_multi_dimensional(&to.start_value).unwrap()
            } else {
                from_value
            },
            ease: from.bezier.clone().map(Into::into),
            path: None,
            metric: None,
        }
    }
}

impl<V, C> MaybeTrack<V, C>
where
    V: FromValue<C>,
    C: en::Num,
{
    pub(crate) fn from_value(value: EitherValue, frame_rate: f64) -> Self {
        match value {
            EitherValue::Fixed(Value { value, .. }) => Self::Fixed(V::from_value(value)),
            EitherValue::Animated(keyframed) => Self::Animated({
                IntervalTrack::from_intervals(keyframed.keyframes.windows(2).map(|window| {
                    Interval::from_value_keyframes([&window[0], &window[1]], frame_rate)
                }))
            }),
        }
    }
}

impl<V, C> MaybeTrack<V, C>
where
    V: FromMultiDimensional<C>,
    C: en::Num,
{
    pub(crate) fn from_multi_dimensional(value: EitherMultiDimensional, frame_rate: f64) -> Self {
        match value {
            EitherMultiDimensional::Fixed(MultiDimensional { value, .. }) => {
                // TODO: no unwraps
                Self::Fixed(V::from_multi_dimensional(&value).unwrap())
            }
            EitherMultiDimensional::Animated(keyframed) => Self::Animated({
                IntervalTrack::from_intervals(keyframed.keyframes.windows(2).map(|window| {
                    Interval::from_offset_keyframes([&window[0], &window[1]], frame_rate)
                }))
            }),
        }
    }
}
