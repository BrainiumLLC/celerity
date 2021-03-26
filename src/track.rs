use std::marker::PhantomData;

use crate::{
    interval::{BezierEase, BezierPath},
    spline::{
        catmull_rom::{catmull_rom_to_bezier, t_values},
        SplineMap,
    },
    Animatable,
};
use gee::en;
use time_point::Duration;

#[derive(Clone, Debug)]
pub enum AnimationStyle<V: Animatable<C>, C: en::Num> {
    Linear,
    Hold,
    Bezier(Option<BezierEase>, BezierPath<V, C>, Option<SplineMap>),
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

    pub fn bezier(frame: Frame<V, C>, ease: BezierEase, path: BezierPath<V, C>) -> Self {
        Self::new(
            frame.offset,
            frame.value,
            AnimationStyle::Bezier(Some(ease), path, None),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Track<V: Animatable<C>, C: en::Num> {
    keyframes: Vec<Keyframe<V, C>>,
}

impl<V: Animatable<C>, C: en::Num> Default for Track<V, C> {
    fn default() -> Self {
        Self { keyframes: vec![] }
    }
}

impl<V: Animatable<C>, C: en::Num> Track<V, C> {
    pub fn new() -> Self {
        Self::default()
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
                let cr_coords = frames.iter().map(|frame| frame.value).collect::<Vec<_>>();

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
                        AnimationStyle::Bezier(
                            None,
                            BezierPath::new(b1, b2),
                            Some(SplineMap::from_bezier(&b0, &b1, &b2, &b3)),
                        ),
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

    pub fn from_keyframe(keyframe: Keyframe<V, C>) -> Self {
        Self::new().with_keyframe(keyframe)
    }

    pub fn from_keyframes(keyframes: impl IntoIterator<Item = Keyframe<V, C>>) -> Self {
        Self::new().with_keyframes(keyframes)
    }

    pub fn with_keyframe(mut self, keyframe: Keyframe<V, C>) -> Self {
        self.add_keyframe(keyframe);
        self
    }

    pub fn with_keyframes(mut self, keyframes: impl IntoIterator<Item = Keyframe<V, C>>) -> Self {
        self.add_keyframes(keyframes);
        self
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe<V, C>) -> &mut Self {
        self.keyframes.push(keyframe);
        self
    }

    pub fn add_keyframes(
        &mut self,
        keyframes: impl IntoIterator<Item = Keyframe<V, C>>,
    ) -> &mut Self {
        self.keyframes.extend(keyframes);
        self
    }
}
