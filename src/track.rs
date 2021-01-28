use std::marker::PhantomData;

use crate::{
    //bezier::{bezier_value, BezierPoints},
    catmullrom::{catmull_rom_time_scale, centripetal_catmull_rom},
    coordinate::Coordinate,
    ease::eased_lerp,
    Animatable,
    Animation,
    BoundedAnimation,
};
use gee::en;
use time_point::Duration;

#[derive(Clone, Debug)]
pub enum AnimationStyle<V: Animatable<T>, T: en::Float> {
    Linear,
    Hold,
    Bezier(PhantomData<V>),
    CatmullRom,
    Eased(fn(T) -> T),
}

#[derive(Clone, Debug)]
pub struct Keyframe<V: Animatable<T>, T: en::Float> {
    pub offset: Duration,
    pub value: V,
    pub style: AnimationStyle<V, T>,
    _marker: PhantomData<T>,
}

impl<V: Animatable<T>, T: en::Float> Keyframe<V, T> {
    pub fn new(offset: Duration, value: V, style: AnimationStyle<V, T>) -> Self {
        Self {
            offset,
            value,
            style,
            _marker: PhantomData,
        }
    }

    pub fn bezier(
        offset: Duration,
        value: V, /*, control_points: BezierPoints<T, V>*/
    ) -> Self {
        Self::new(
            offset,
            value,
            AnimationStyle::Bezier(PhantomData), //(control_points),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Track<V: Animatable<T> + en::Num, T: en::Float> {
    keyframes: Vec<(Keyframe<V, T>, Duration)>,
}

impl<V: Animatable<T> + en::Num, T: en::Float> Animation<V, T> for Track<V, T> {
    fn sample(&self, elapsed: Duration) -> V {
        if let Some((next_frame, next_abs_offset)) = self.next_upcoming_frame(&elapsed) {
            let (last_frame, last_abs_offset) = self.last_elapsed_frame(&elapsed).unwrap();
            let style = &last_frame.style;
            let relative_elapsed = en::cast::<f64, _>(elapsed.nanos - last_abs_offset.nanos)
                / en::cast::<f64, _>(next_abs_offset.nanos - last_abs_offset.nanos);

            match style {
                AnimationStyle::Linear => last_frame.value.lerp(
                    next_frame.value,
                    en::cast(relative_elapsed),
                ),
                AnimationStyle::Hold => last_frame.value,
                AnimationStyle::Bezier(PhantomData)/*(control_points)*/ => //bezier_value(
                    last_frame.value,
                //     next_frame.value,
                //     control_points,
                //     elapsed,
                // ),
                AnimationStyle::CatmullRom => self.catmull_rom_sample(elapsed),
                AnimationStyle::Eased(ease) => self.eased_sample(elapsed, *ease),
            }
        } else {
            self.last_elapsed_frame(&elapsed).unwrap().0.value
        }
    }
}

impl<V: Animatable<T> + en::Num, T: en::Float> BoundedAnimation<V, T> for Track<V, T> {
    fn duration(&self) -> Duration {
        self.keyframes
            .last()
            .map(|(_, duration)| *duration)
            .unwrap_or_default()
    }
}

impl<V: Animatable<T> + en::Num, T: en::Float> Track<V, T> {
    pub fn new() -> Self {
        Self { keyframes: vec![] }
    }

    pub fn from_keyframes(keyframes: Vec<Keyframe<V, T>>) -> Self {
        assert_eq!(
            keyframes[0].offset.nanos, 0,
            "Initial keyframe should have 0 time offset"
        );
        Self::new().with_keyframes(keyframes)
    }

    fn eased_sample(&self, elapsed: Duration, easing: fn(T) -> T) -> V {
        let (a_value, a_abs_offset) = self
            .keyframes
            .iter()
            .filter(|(_, abs_offset)| *abs_offset <= elapsed)
            .last()
            .map(|(kf, abs_offset)| (kf.value, *abs_offset))
            .unwrap_or_else(|| (self.initial_frame().0.value, Duration::new(0)));
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
            eased_lerp(a_value, b_kf.value, en::cast(f), easing)
        } else {
            a_value
        }
    }

    fn catmull_rom_sample(&self, elapsed: Duration) -> V
    where
        V: en::Num,
    {
        // Step 1: Determine your four control points (last two elapsed frames, next two unelapsed)
        let elapsed_frame_count = self.elapsed_frames(&elapsed).len();
        let upcoming_frame_count = self.upcoming_frames(&elapsed).len();

        if upcoming_frame_count != 0 {
            let f0 = match elapsed_frame_count {
                0 => self.initial_frame(),
                1 => self.last_elapsed_frame(&elapsed).unwrap(),
                _ => self.second_to_last_elapsed_frame(&elapsed).unwrap(),
            };

            let f1 = match elapsed_frame_count {
                0 => f0, // TODO: points can't be duplicated
                _ => self.last_elapsed_frame(&elapsed).unwrap(),
            };

            let f2 = self.next_upcoming_frame(&elapsed).unwrap();

            let f3 = if upcoming_frame_count > 1 {
                self.second_next_upcoming_frame(&elapsed).unwrap()
            } else {
                f2 // TODO: points can't be duplicated
            };

            let scaled_elapsed = catmull_rom_time_scale(f0.1, f1.1, f2.1, f3.1, elapsed);
            let segment_elapsed = /*scaled_*/elapsed - f1.1;
            let segment_duration = f2.1 - f1.1;
            let relative_elapsed = segment_elapsed.nanos as f64 / segment_duration.nanos as f64;
            let cr_out = centripetal_catmull_rom::<V, T>(
                Coordinate::new(en::cast(f0.0.value), en::cast(f0.1.nanos)),
                Coordinate::new(f1.0.value, en::cast(f1.1.nanos)),
                Coordinate::new(f2.0.value, en::cast(f2.1.nanos)),
                Coordinate::new(f3.0.value, en::cast(f3.1.nanos)),
                relative_elapsed,
            );
            cr_out.x
        } else {
            self.keyframes.iter().last().unwrap().0.value
        }
    }

    fn initial_frame(&self) -> &(Keyframe<V, T>, Duration) {
        &self.keyframes[0]
    }

    fn elapsed_frames(&self, elapsed: &Duration) -> Vec<&(Keyframe<V, T>, Duration)> {
        self.keyframes
            .iter()
            .filter(|(_, abs_offset)| abs_offset <= elapsed)
            .collect()
    }

    pub fn last_elapsed_frame(&self, elapsed: &Duration) -> Option<&(Keyframe<V, T>, Duration)> {
        self.keyframes
            .iter()
            .filter(|(_, abs_offset)| abs_offset <= elapsed)
            .last()
    }

    fn second_to_last_elapsed_frame(
        &self,
        elapsed: &Duration,
    ) -> Option<&(Keyframe<V, T>, Duration)> {
        let elapsed_frames = self.elapsed_frames(elapsed);
        let count = elapsed_frames.len();

        match self.elapsed_frames(elapsed).len() {
            0 => None,
            _ => Some(elapsed_frames[count - 2]),
        }
    }

    fn upcoming_frames(&self, elapsed: &Duration) -> Vec<&(Keyframe<V, T>, Duration)> {
        self.keyframes
            .iter()
            .filter(|(_, abs_offset)| abs_offset > elapsed)
            .collect()
    }

    pub fn next_upcoming_frame(&self, elapsed: &Duration) -> Option<&(Keyframe<V, T>, Duration)> {
        self.keyframes
            .iter()
            .find(|(_, abs_offset)| abs_offset > elapsed)
    }

    fn second_next_upcoming_frame(
        &self,
        elapsed: &Duration,
    ) -> Option<&(Keyframe<V, T>, Duration)> {
        let upcoming_frames = self.upcoming_frames(elapsed);

        if upcoming_frames.len() > 1 {
            Some(upcoming_frames[1])
        } else {
            None
        }
    }

    // pub fn from_keyframe(keyframe: Keyframe<T>) -> Self {
    //     Self::new().with_keyframe(keyframe)
    // }

    // pub fn from_keyframes(keyframes: impl IntoIterator<Item = Keyframe<T>>) -> Self {
    //     Self::new().with_keyframes(keyframes)
    // }

    pub fn with_keyframe(mut self, keyframe: Keyframe<V, T>) -> Self {
        self.add_keyframe(keyframe);
        self
    }

    pub fn with_keyframes(mut self, keyframes: impl IntoIterator<Item = Keyframe<V, T>>) -> Self {
        self.add_keyframes(keyframes);
        self
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe<V, T>) -> &mut Self {
        let abs_offset = self.duration() + keyframe.offset;
        self.keyframes.push((keyframe, abs_offset));
        self
    }

    pub fn add_keyframes(
        &mut self,
        keyframes: impl IntoIterator<Item = Keyframe<V, T>>,
    ) -> &mut Self {
        for keyframe in keyframes {
            self.add_keyframe(keyframe);
        }
        self
    }
}
