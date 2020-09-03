use crate::{Animation, BoundedAnimation, Output};
use gee::en;
use time_point::{Duration, TimePoint};

#[derive(Clone, Debug)]
pub struct Keyframe<O: Output<T>, T: en::Float> {
    offset: Duration,
    value: O,
    easing: fn(T) -> T,
}

impl<O: Output<T>, T: en::Float> Keyframe<O, T> {
    pub fn new(offset: Duration, value: O, easing: fn(T) -> T) -> Self {
        Self {
            offset,
            value,
            easing,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Path<O: Output<T>, T: en::Float> {
    initial: O,
    duration: Duration,
    keyframes: Vec<Keyframe<O, T>>,
}

impl<O: Output<T>, T: en::Float> Animation<O, T> for Path<O, T> {
    fn sample(&mut self, start: TimePoint, time: TimePoint) -> O {
        assert_start_lte_time!(Path, start, time);
        let a = self
            .keyframes
            .iter()
            .find_map(|kf| Some((start + kf.offset, kf.value)).filter(|(t, _)| *t <= time))
            .unwrap_or_else(|| (start, self.initial));
        let b = self
            .keyframes
            .iter()
            .find_map(|kf| Some((start + kf.offset, kf)).filter(|(t, _)| *t > time));
        if let Some((bt, b)) = b {
            let f = 1.0 - (bt - time).as_secs_f64() / (bt - a.0).as_secs_f64();
            debug_assert!(f >= 0.0, "f was {}, but must not be less than 0.0", f);
            debug_assert!(f <= 1.0, "f was {}, but must not be greater than 1.0", f);
            a.1.eased_lerp(b.value, en::cast(f), b.easing)
        } else {
            a.1
        }
    }
}

impl<O: Output<T>, T: en::Float> BoundedAnimation<O, T> for Path<O, T> {
    fn duration(&self) -> Duration {
        self.duration
    }
}

impl<O: Output<T>, T: en::Float> Path<O, T> {
    pub fn new(initial: O) -> Self {
        Self {
            initial,
            duration: Default::default(),
            keyframes: Default::default(),
        }
    }

    // pub fn from_keyframe(keyframe: Keyframe<T>) -> Self {
    //     Self::new().with_keyframe(keyframe)
    // }

    // pub fn from_keyframes(keyframes: impl IntoIterator<Item = Keyframe<T>>) -> Self {
    //     Self::new().with_keyframes(keyframes)
    // }

    pub fn with_keyframe(mut self, keyframe: Keyframe<O, T>) -> Self {
        self.add_keyframe(keyframe);
        self
    }

    pub fn with_keyframes(mut self, keyframes: impl IntoIterator<Item = Keyframe<O, T>>) -> Self {
        self.add_keyframes(keyframes);
        self
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe<O, T>) -> &mut Self {
        self.duration += keyframe.offset;
        self.keyframes.push(keyframe);
        self
    }

    pub fn add_keyframes(
        &mut self,
        keyframes: impl IntoIterator<Item = Keyframe<O, T>>,
    ) -> &mut Self {
        for keyframe in keyframes {
            self.add_keyframe(keyframe);
        }
        self
    }
}
