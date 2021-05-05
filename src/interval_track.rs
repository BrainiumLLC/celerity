use crate::{
    interval::Interval,
    spline::{
        bezier_ease::BezierEase, bezier_path::BezierPath,
        catmull_rom::centripetal_catmull_rom_to_bezier, SplineMap,
    },
    Animatable, Animation, AnimationStyle, BoundedAnimation, Frame, Keyframe,
};
use time_point::Duration;

#[derive(Clone, Debug)]
pub struct IntervalTrack<V: Animatable> {
    intervals: Vec<Interval<V>>,
    track_ease: Option<BezierEase>,
}

impl<V: Animatable> IntervalTrack<V> {
    pub fn new() -> Self {
        Self {
            intervals: vec![],
            track_ease: None,
        }
    }

    pub fn from_intervals(intervals: impl IntoIterator<Item = Interval<V>>) -> Self {
        Self::new().with_intervals(intervals)
    }

    pub fn from_duration_and_values(
        start: Duration,
        duration: Duration,
        values: Vec<V>,
        track_ease: Option<BezierEase>,
    ) -> Self {
        match values.len() {
            0 => IntervalTrack::new(),
            1 => IntervalTrack::new().with_interval(Interval::hold(values[0])),
            2 => IntervalTrack::new().with_interval(Interval::linear(
                Frame::new(start, values[0]),
                Frame::new(start + duration, values[1]),
                None,
            )),
            _ => {
                // Add first/last values to refine animation path
                let bookended_values = bookend(values, BookendStyle::Linear);
                // Calculate BezierPath and SplineMap for each interval
                let (paths, maps) = values_to_bezier_structs(&bookended_values);
                // Calculate durations for each interval threshold
                let durations = constant_velocity_durations(&accumulate_lengths(&maps), duration);

                IntervalTrack::from_intervals(
                    bookended_values
                        .windows(2)
                        .skip(1)
                        .zip(durations.windows(2))
                        .zip(paths)
                        .zip(maps)
                        .map(|(((value_window, duration_window), path), map)| {
                            Interval::new(
                                duration_window[0],
                                duration_window[1],
                                value_window[0],
                                value_window[1],
                                None,
                                Some(path),
                                Some(map),
                            )
                        }),
                )
                .with_track_ease(track_ease)
            }
        }
    }

    pub fn auto_bezier(frames: Vec<Frame<V>>) -> Self {
        // TODO: How to get rid of this?
        let mut acc_elapsed = Duration::zero();

        Self::from_intervals(bookend_frames(frames, BookendStyle::Repeat).windows(4).map(
            |window| {
                let (b0, b1, b2, b3) = centripetal_catmull_rom_to_bezier(
                    &window[0].value,
                    &window[1].value,
                    &window[2].value,
                    &window[3].value,
                );
                acc_elapsed = acc_elapsed + window[2].offset;
                Interval::new(
                    acc_elapsed - window[2].offset,
                    acc_elapsed,
                    window[1].value,
                    window[2].value,
                    None,
                    Some(BezierPath::new(b1, b2)),
                    Some(SplineMap::from_bezier(&b0, &b1, &b2, &b3)),
                )
            },
        ))
    }

    pub fn from_keyframes(keyframes: Vec<Keyframe<V>>) -> Self {
        match keyframes.len() {
            0 => Self::new(),
            1 => Self::new().with_interval(Interval::hold(keyframes[0].value)),
            _ => {
                // TODO: Is there a better way to accumulate a value while mapping over an iterator?
                let mut acc_offset = Duration::zero();
                Self::new().with_intervals(
                    keyframes
                        .windows(2)
                        .map(|window| {
                            acc_offset = acc_offset + window[0].offset;
                            match &window[0].style {
                                AnimationStyle::Hold => Interval::new(
                                    acc_offset,
                                    acc_offset + window[1].offset,
                                    window[0].value,
                                    window[0].value,
                                    None,
                                    None,
                                    None,
                                ),
                                AnimationStyle::Linear => Interval::new(
                                    acc_offset,
                                    acc_offset + window[1].offset,
                                    window[0].value,
                                    window[1].value,
                                    None,
                                    None,
                                    None,
                                ),
                                AnimationStyle::Bezier(ease, path, metric) => Interval::new(
                                    acc_offset,
                                    acc_offset + window[1].offset,
                                    window[0].value,
                                    window[1].value,
                                    ease.clone(),
                                    Some(path.clone()),
                                    metric.clone(),
                                ),
                                AnimationStyle::Eased(ease) => Interval::new(
                                    acc_offset,
                                    acc_offset + window[1].offset,
                                    window[0].value,
                                    window[1].value,
                                    None,
                                    None,
                                    Some(SplineMap::from_spline(ease)),
                                ),
                            }
                        })
                        .collect::<Vec<Interval<V>>>(),
                ) // TODO: Is the collect() here strictly necessary?
            }
        }
    }

    pub fn with_track_ease(mut self, track_ease: Option<BezierEase>) -> Self {
        self.track_ease = track_ease;
        self
    }

    pub fn with_interval(mut self, interval: Interval<V>) -> Self {
        self.add_interval(interval);
        self
    }

    pub fn with_intervals(mut self, intervals: impl IntoIterator<Item = Interval<V>>) -> Self {
        self.add_intervals(intervals);
        self
    }

    pub fn add_interval(&mut self, interval: Interval<V>) -> &mut Self {
        self.intervals.push(interval);
        self
    }

    pub fn add_intervals(&mut self, intervals: impl IntoIterator<Item = Interval<V>>) -> &mut Self {
        for interval in intervals {
            self.add_interval(interval);
        }
        self
    }

    pub fn current_interval(&self, elapsed: &Duration) -> Option<&Interval<V>> {
        self.intervals
            .iter()
            .find(|interval| interval.end > *elapsed)
            .or_else(|| self.intervals.last())
    }
}

impl<V: Animatable> Animation<V> for IntervalTrack<V> {
    fn sample(&self, elapsed: Duration) -> V {
        match &self.track_ease {
            Some(ease) => {
                let eased_elapsed = self.duration()
                    * ease.ease(
                        (elapsed - self.intervals[0].start).as_secs_f64()
                            / self.duration().as_secs_f64(),
                    );
                self.current_interval(&eased_elapsed)
                    .unwrap()
                    .sample(eased_elapsed)
            }
            None => self.current_interval(&elapsed).unwrap().sample(elapsed),
        }
    }
}

impl<V: Animatable> BoundedAnimation<V> for IntervalTrack<V> {
    fn duration(&self) -> Duration {
        self.intervals.last().unwrap().end - self.intervals[0].start
    }
}

// Different ways of selecting additional control points at either end of a series of values.
pub enum BookendStyle {
    // Repeat the first and last values
    Repeat,
    // Linearly extrapolate using the first two and last two values
    Linear,
    // Use the first/last three points to calculate a point that would loop back toward the second-to-first/last point
    Spiral,
}

pub fn bookend<V: Animatable>(values: Vec<V>, style: BookendStyle) -> Vec<V> {
    match style {
        BookendStyle::Repeat => {
            let final_bookend = *values.last().unwrap();
            std::iter::once(values[0])
                .chain(values)
                .chain(std::iter::once(final_bookend))
                .collect()
        }
        BookendStyle::Linear => {
            let last_index = values.len() - 1;
            let initial_bookend = values[0].sub(values[1]);
            let final_bookend = values[last_index].sub(values[last_index - 1]);

            std::iter::once(initial_bookend)
                .chain(values)
                .chain(std::iter::once(final_bookend))
                .collect()
        }
        BookendStyle::Spiral => {
            let last_index = values.len() - 1;
            let initial_bookend = values[0].sub(values[1].sub(values[2]));
            let final_bookend =
                values[last_index].sub(values[last_index - 1].sub(values[last_index - 2]));

            std::iter::once(initial_bookend)
                .chain(values)
                .chain(std::iter::once(final_bookend))
                .collect()
        }
    }
}

pub fn bookend_frames<V: Animatable>(frames: Vec<Frame<V>>, style: BookendStyle) -> Vec<Frame<V>> {
    let bookended_values = bookend(frames.iter().map(|frame| frame.value).collect(), style);
    let bookended_durations = std::iter::once(Duration::zero())
        .chain(frames.into_iter().map(|frame| frame.offset))
        .chain(std::iter::once(Duration::zero()))
        .collect::<Vec<Duration>>();
    bookended_values
        .iter()
        .zip(bookended_durations)
        .map(|(v, d)| Frame::new(d, *v))
        .collect()
}

fn values_to_bezier_structs<V: Animatable>(
    values: &Vec<V>,
) -> (Vec<BezierPath<V>>, Vec<SplineMap>) {
    values
        .windows(4)
        .map(|window| {
            let (b0, b1, b2, b3) =
                centripetal_catmull_rom_to_bezier(&window[0], &window[1], &window[2], &window[3]);
            (
                BezierPath::new(b1, b2),
                SplineMap::from_bezier(&b0, &b1, &b2, &b3),
            )
        })
        .unzip()
}

fn accumulate_lengths(maps: &Vec<SplineMap>) -> Vec<f64> {
    // Does not include initial state
    // maps.iter().scan(0.0, |length, map| {
    //     *length = *length + map.length;
    //     Some(*length)
    // }).collect()

    // Does not include final state
    // maps.iter().scan(0.0, |length, map| {
    //     let result = Some(*length);
    //     *length = *length + map.length;
    //     result
    // }).collect()

    // Ugly, uses mut
    let mut accumulated_lengths = vec![];
    let total_length = maps.iter().fold(0.0, |len, map| {
        accumulated_lengths.push(len);
        len + map.length
    });
    accumulated_lengths.push(total_length);

    accumulated_lengths
}

fn constant_velocity_durations(distances: &Vec<f64>, duration: Duration) -> Vec<Duration> {
    distances
        .iter()
        .map(|distance| (distance / distances.last().unwrap()) * duration)
        .collect()
}
