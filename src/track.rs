use crate::{ease::eased_lerp, Animation, BoundedAnimation, Output};
use gee::en;
use time_point::Duration;

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
pub struct Track<O: Output<T>, T: en::Float> {
    initial: O,
    keyframes: Vec<(Keyframe<O, T>, Duration)>,
}

impl<O: Output<T>, T: en::Float> Animation<O, T> for Track<O, T> {
    fn sample(&mut self, elapsed: Duration) -> O {
        let (a_value, a_abs_offset) = self
            .keyframes
            .iter()
            .find(|(_, abs_offset)| *abs_offset <= elapsed)
            .map(|(kf, abs_offset)| (kf.value, *abs_offset))
            .unwrap_or_else(|| (self.initial, Duration::zero()));
        let b = self
            .keyframes
            .iter()
            .find(|(_, abs_offset)| *abs_offset > elapsed)
            .cloned();
        if let Some((b_kf, b_abs_offset)) = b {
            let f = 1.0
                - (b_abs_offset - elapsed).as_secs_f64()
                    / (b_abs_offset - a_abs_offset).as_secs_f64();
            debug_assert!(f >= 0.0, "f was {}, but must not be less than 0.0", f);
            debug_assert!(f <= 1.0, "f was {}, but must not be greater than 1.0", f);
            eased_lerp(a_value, b_kf.value, en::cast(f), b_kf.easing)
        } else {
            a_value
        }
    }
}

impl<O: Output<T>, T: en::Float> BoundedAnimation<O, T> for Track<O, T> {
    fn duration(&self) -> Duration {
        self.keyframes
            .last()
            .map(|(_, duration)| *duration)
            .unwrap_or_default()
    }
}

impl<O: Output<T>, T: en::Float> Track<O, T> {
    pub fn new(initial: O) -> Self {
        Self {
            initial,
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
        let abs_offset = self.duration() + keyframe.offset;
        self.keyframes.push((keyframe, abs_offset));
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
