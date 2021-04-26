use super::conv::{FromMultiDimensional, FromValue};
use crate::{
    interval::{BezierEase, Interval, IntervalTrack},
    Animatable, Animation,
};
use bodymovin::properties::{EitherMultiDimensional, EitherValue, MultiDimensional, Value};
use gee::en;
use std::marker::PhantomData;
use time_point::Duration;

impl From<bodymovin::properties::Bezier1d> for BezierEase {
    fn from(bezier: bodymovin::properties::Bezier1d) -> Self {
        Self {
            ox: bezier.out_value.x,
            oy: bezier.out_value.y,
            ix: bezier.in_value.x,
            iy: bezier.in_value.y,
        }
    }
}

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
    ) -> Result<Interval<V, C>, V::Error> {
        let from_value = V::from_value(from.start_value)?;
        let to_value = if !from.hold {
            V::from_value(to.start_value)?
        } else {
            from_value
        };
        Ok(Interval {
            start: Duration::from_secs_f64(from.start_time / frame_rate),
            end: Duration::from_secs_f64(to.start_time / frame_rate),
            from: from_value,
            to: to_value,
            ease: from.bezier.clone().map(Into::into),
            path: None,
            metric: None,
        })
    }
}

impl<V: FromMultiDimensional<C>, C: en::Num> Interval<V, C> {
    pub(crate) fn from_offset_keyframes(
        [from, to]: [&bodymovin::properties::OffsetKeyframe; 2],
        frame_rate: f64,
    ) -> Result<Interval<V, C>, V::Error> {
        let from_value = V::from_multi_dimensional(&from.start_value)?;
        let to_value = if !from.hold {
            V::from_multi_dimensional(&to.start_value)?
        } else {
            from_value
        };
        Ok(Interval {
            start: Duration::from_secs_f64(from.start_time / frame_rate),
            end: Duration::from_secs_f64(to.start_time / frame_rate),
            from: from_value,
            to: to_value,
            ease: from.bezier.clone().map(Into::into),
            path: None,
            metric: None,
        })
    }
}

impl<V, C> MaybeTrack<V, C>
where
    V: FromValue<C>,
    C: en::Num,
{
    pub(crate) fn from_value(value: EitherValue, frame_rate: f64) -> Result<Self, V::Error> {
        match value {
            EitherValue::Fixed(Value { value, .. }) => V::from_value(value).map(Self::Fixed),
            EitherValue::Animated(keyframed) => keyframed
                .keyframes
                .windows(2)
                .map(|window| Interval::from_value_keyframes([&window[0], &window[1]], frame_rate))
                .collect::<Result<Vec<_>, _>>()
                .map(IntervalTrack::from_intervals)
                .map(Self::Animated),
        }
    }
}

impl<V, C> MaybeTrack<V, C>
where
    V: FromMultiDimensional<C>,
    C: en::Num,
{
    pub(crate) fn from_multi_dimensional(
        value: EitherMultiDimensional,
        frame_rate: f64,
    ) -> Result<Self, V::Error> {
        match value {
            EitherMultiDimensional::Fixed(MultiDimensional { value, .. }) => {
                V::from_multi_dimensional(&value).map(Self::Fixed)
            }
            EitherMultiDimensional::Animated(keyframed) => keyframed
                .keyframes
                .windows(2)
                .map(|window| Interval::from_offset_keyframes([&window[0], &window[1]], frame_rate))
                .collect::<Result<Vec<_>, _>>()
                .map(IntervalTrack::from_intervals)
                .map(Self::Animated),
        }
    }
}
