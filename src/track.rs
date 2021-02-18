use std::marker::PhantomData;

use crate::{
    catmullrom::{
        bezier_value, catmull_rom_time_scale, catmull_rom_to_bezier, centripetal_catmull_rom,
        t_values,
    },
    coordinate::Coordinate,
    ease::{
        bezier::{self, cubic_bezier_ease},
        eased_lerp,
    },
    Animatable, Animation, BoundedAnimation,
};
use gee::en;
use time_point::Duration;

#[derive(Clone, Debug)]
pub enum AnimationStyle<V: Animatable<C>, C: en::Num> {
    Linear,
    Hold,
    Bezier(BezierPath<V, C>), // TODO: ease
    Eased(fn(f64) -> f64),
}

// Basic frame w/o AnimationStyle (not sold on the name)
#[derive(Copy, Clone, Debug)]
pub struct Frame<V: Animatable<C>, C: en::Num> {
    pub offset: Duration,
    pub value: V,
    _marker: PhantomData<C>,
}

impl<V: Animatable<C>, C: en::Num> Frame<V, C> {
    pub fn new(offset: Duration, value: V) -> Self {
        Self {
            offset,
            value,
            _marker: PhantomData,
        }
    }
}

// TODO: use Frame here?
#[derive(Clone, Debug)]
pub struct Keyframe<V: Animatable<C>, C: en::Num> {
    pub offset: Duration,
    pub value: V,
    pub style: AnimationStyle<V, C>,
    _marker: PhantomData<C>,
}

impl<V: Animatable<C>, C: en::Num> Keyframe<V, C> {
    pub fn new(offset: Duration, value: V, style: AnimationStyle<V, C>) -> Self {
        Self {
            offset,
            value,
            style,
            _marker: PhantomData,
        }
    }

    pub fn linear(frame: Frame<V, C>) -> Self {
        Self::new(frame.offset, frame.value, AnimationStyle::Linear)
    }

    pub fn hold(frame: Frame<V, C>) -> Self {
        Self::new(frame.offset, frame.value, AnimationStyle::Hold)
    }

    pub fn bezier(frame: Frame<V, C>, control_points: BezierPath<V, C>) -> Self {
        Self::new(
            frame.offset,
            frame.value,
            AnimationStyle::Bezier(control_points),
        )
    }
}

// Describes the Bezier ease between two Animatables
#[derive(Debug, Clone, Copy)]
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


#[derive(Clone, Copy, Debug)]
pub struct BezierPath<V: Animatable<C>, C: en::Num> {
    pub ov: V,
    pub iv: V,
    _marker: PhantomData<C>,
}

impl<V: Animatable<C>, C: en::Num> BezierPath<V, C> {
    pub fn new (b0: V, b1: V, b2: V, b3: V) -> Self {
        let dv1v0 = b0.zip_map(b1, |v0, v1| v1 - v0);
        let dv2v0 = b0.zip_map(b2, |v0, v2| v2 - v0);
        let dv3v0 = b0.zip_map(b3, |v0, v3| v3 - v0);

        let ov = dv1v0.zip_map(dv3v0, |v10, v30| v10 / v30);
        let iv = dv2v0.zip_map(dv3v0, |v20, v30| v20 / v30);

        Self {ov, iv, _marker: PhantomData}
    }
}

#[derive(Clone, Debug)]
pub struct Track<V: Animatable<C>, C: en::Num> {
    keyframes: Vec<(Keyframe<V, C>, Duration)>,
}

impl<V: Animatable<C>, C: en::Num> Animation<V, C> for Track<V, C> {
    fn sample(&self, elapsed: Duration) -> V {
        if let Some((next_frame, next_abs_offset)) = self.next_upcoming_frame(&elapsed) {
            let (last_frame, last_abs_offset) = self.last_elapsed_frame(&elapsed).unwrap();
            let style = &last_frame.style;
            let relative_elapsed = en::cast::<f64, _>(elapsed.nanos - last_abs_offset.nanos)
                / en::cast::<f64, _>(next_abs_offset.nanos - last_abs_offset.nanos);

            match style {
                AnimationStyle::Linear => last_frame
                    .value
                    .lerp(next_frame.value, en::cast(relative_elapsed)),
                AnimationStyle::Hold => last_frame.value,
                AnimationStyle::Bezier(control_points) => self.bezier_sample(
                    last_frame,
                    next_frame,
                    control_points,
                    elapsed - *last_abs_offset,
                ),
                AnimationStyle::Eased(ease) => self.eased_sample(elapsed, *ease),
            }
        } else {
            self.last_elapsed_frame(&elapsed).unwrap().0.value
        }
    }
}

impl<V: Animatable<C>, C: en::Num> BoundedAnimation<V, C> for Track<V, C> {
    fn duration(&self) -> Duration {
        self.keyframes
            .last()
            .map(|(_, duration)| *duration)
            .unwrap_or_default()
    }
}

impl<V: Animatable<C>, C: en::Num> Track<V, C> {
    pub fn new() -> Self {
        Self { keyframes: vec![] }
    }

    pub fn from_keyframes(keyframes: Vec<Keyframe<V, C>>) -> Self {
        assert_eq!(
            keyframes[0].offset.nanos, 0,
            "Initial keyframe should have 0 time offset"
        );
        Self::new().with_keyframes(keyframes)
    }

    // Centripetal Catmull-Rom == Auto Bezier
    pub fn catmull_rom(frames: Vec<Frame<V, C>>) -> Self
    where
        V: Animatable<C>,
        C: en::Num,
    {
        Self::auto_bezier(frames)
    }

    pub fn auto_bezier(frames: Vec<Frame<V, C>>) -> Self
    where
        V: Animatable<C>,
        C: en::Num,
    {
        match frames.len() {
            0 => Self::new(),
            1 => Self::from_keyframes(vec![Keyframe::hold(frames[0])]),
            2 => Self::from_keyframes(vec![Keyframe::linear(frames[0]), Keyframe::hold(frames[1])]),
            _ => {
                // Gather coordinate values from frames
                let cr_coords = frames
                    .iter()
                    .map(|frame| frame.value)
                    .collect::<Vec<_>>();

                // Construct Bezier Keyframes using Catmull-Rom spline
                let mut keyframes = vec![];
                for i in 0..cr_coords.len() - 1 {
                    // Determine Catmull-Rom (cr) coordinates for current segment
                    let cr0 = if i == 0 {
                        cr_coords[0] // We may want to augment this point to influence our animation
                    } else {
                        cr_coords[i - 1]
                    };
                    let cr1 = cr_coords[i];
                    let cr2 = cr_coords[i + 1];
                    let cr3 = if i == cr_coords.len() - 2 {
                        cr_coords[i + 1] // We may want to augment this point to influence our animation
                    } else {
                        cr_coords[i + 2]
                    };

                    // Determine Bezier control points
                    let (t0, t1, t2, t3) = t_values(&cr0, &cr1, &cr2, &cr3, 0.5);
                    let (b0, b1, b2, b3) =
                        catmull_rom_to_bezier(&cr0, &cr1, &cr2, &cr3, t0, t1, t2, t3);

                    // Bezier Keyframe
                    // (BezierPath now describes control points for position, not easing (TODO: easing))
                    keyframes.push(Keyframe::new(
                        frames[i].offset,
                        b0,
                        AnimationStyle::Bezier(BezierPath::new(b0, b1, b2, b3)),
                    ));
                }

                // Hold frame at the end of the track contains final value/duration
                keyframes.push(Keyframe::new(
                    frames.last().unwrap().offset,
                    frames.last().unwrap().value,
                    AnimationStyle::Hold,
                ));

                Self::from_keyframes(keyframes)
            }
        }
    }

    fn bezier_sample(
        &self,
        last_frame: &Keyframe<V, C>,
        next_frame: &Keyframe<V, C>,
        control_points: &BezierPath<V, C>,
        last_frame_elapsed: Duration,
    ) -> V {
        let relative_elapsed = en::cast::<f64, _>(last_frame_elapsed.nanos)
            / en::cast::<f64, _>(next_frame.offset.nanos);

        // TODO: easing
        let eased_elapsed = relative_elapsed;//cubic_bezier_ease(
        //     control_points.ox,
        //     control_points.oy,
        //     control_points.ix,
        //     control_points.iy,
        //     relative_elapsed,
        // );

        last_frame
            .value
            .lerp(next_frame.value, en::cast(eased_elapsed))
    }

    fn eased_sample(&self, elapsed: Duration, easing: fn(f64) -> f64) -> V {
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

    fn initial_frame(&self) -> &(Keyframe<V, C>, Duration) {
        &self.keyframes[0]
    }

    fn elapsed_frames(&self, elapsed: &Duration) -> Vec<&(Keyframe<V, C>, Duration)> {
        self.keyframes
            .iter()
            .filter(|(_, abs_offset)| abs_offset <= elapsed)
            .collect()
    }

    pub fn last_elapsed_frame(&self, elapsed: &Duration) -> Option<&(Keyframe<V, C>, Duration)> {
        self.keyframes
            .iter()
            .filter(|(_, abs_offset)| abs_offset <= elapsed)
            .last()
    }

    fn upcoming_frames(&self, elapsed: &Duration) -> Vec<&(Keyframe<V, C>, Duration)> {
        self.keyframes
            .iter()
            .filter(|(_, abs_offset)| abs_offset > elapsed)
            .collect()
    }

    pub fn next_upcoming_frame(&self, elapsed: &Duration) -> Option<&(Keyframe<V, C>, Duration)> {
        self.keyframes
            .iter()
            .find(|(_, abs_offset)| abs_offset > elapsed)
    }

    // pub fn from_keyframe(keyframe: Keyframe<T>) -> Self {
    //     Self::new().with_keyframe(keyframe)
    // }

    // pub fn from_keyframes(keyframes: impl IntoIterator<Item = Keyframe<T>>) -> Self {
    //     Self::new().with_keyframes(keyframes)
    // }

    pub fn with_keyframe(mut self, keyframe: Keyframe<V, C>) -> Self {
        self.add_keyframe(keyframe);
        self
    }

    pub fn with_keyframes(mut self, keyframes: impl IntoIterator<Item = Keyframe<V, C>>) -> Self {
        self.add_keyframes(keyframes);
        self
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe<V, C>) -> &mut Self {
        let abs_offset = self.duration() + keyframe.offset;
        self.keyframes.push((keyframe, abs_offset));
        self
    }

    pub fn add_keyframes(
        &mut self,
        keyframes: impl IntoIterator<Item = Keyframe<V, C>>,
    ) -> &mut Self {
        for keyframe in keyframes {
            self.add_keyframe(keyframe);
        }
        self
    }
}
