use gee::en::Num;

use crate::{
    ease::Ease,
    interval::{Frame, Interval},
    spline::{bezier_path::BezierPath, catmull_rom::centripetal_catmull_rom_to_bezier, SplineMap},
    Animatable, Animation, BoundedAnimation,
};
use core::fmt::Debug;
use std::time::Duration;

#[derive(Clone)]
pub struct IntervalTrack<V: Animatable> {
    intervals: Vec<Interval<V>>,
    track_ease: Option<Ease>,
}

impl<V: Animatable> IntervalTrack<V> {
    pub fn new() -> Self {
        Self {
            intervals: vec![],
            track_ease: None,
        }
    }

    pub fn from_interval(interval: Interval<V>) -> Self {
        Self::new().with_interval(interval)
    }

    pub fn from_intervals(intervals: impl IntoIterator<Item = Interval<V>>) -> Self {
        Self::new().with_intervals(intervals)
    }

    pub fn from_values(duration: Duration, values: Vec<V>, track_ease: Option<Ease>) -> Self {
        match values.len() {
            0 => IntervalTrack::new(),
            1 => IntervalTrack::from_interval(Interval::hold(values[0], Duration::ZERO)),
            2 => IntervalTrack::from_interval(Interval::eased(
                Frame::new(Duration::ZERO, values[0]),
                Frame::new(duration, values[1]),
                track_ease,
            )),
            _ => {
                let lengths = values
                    .windows(2)
                    .map(|window| window[0].distance_to(window[1]))
                    .collect();

                let durations =
                    constant_velocity_durations(&accumulate_lengths(&lengths), duration);

                IntervalTrack::from_intervals(values.windows(2).zip(durations.windows(2)).map(
                    |(value_window, duration_window)| {
                        Interval::new(
                            duration_window[0],
                            duration_window[1],
                            value_window[0],
                            value_window[1],
                            None,
                            None,
                            None,
                        )
                    },
                ))
                .with_track_ease(track_ease)
            }
        }
    }

    pub fn path(
        duration: Duration,
        values: Vec<V>,
        bookend_style: BookendStyle,
        track_ease: Option<Ease>,
        rectify: bool,
    ) -> Self {
        match values.len() {
            0 => IntervalTrack::new(),
            1 => IntervalTrack::from_interval(Interval::hold(values[0], Duration::ZERO)),
            2 => IntervalTrack::from_interval(Interval::eased(
                Frame::new(Duration::ZERO, values[0]),
                Frame::new(duration, values[1]),
                track_ease,
            )),
            _ => {
                // Add first/last values to refine animation path
                let bookended_values = bookend(values, bookend_style);
                // Calculate BezierPath and SplineMap for each interval
                let (paths, maps) = bookended_values_to_bezier_structs(&bookended_values, rectify);
                // Calculate durations for each interval threshold
                let lengths = maps.iter().map(|map| map.length).collect::<Vec<_>>();
                let durations =
                    constant_velocity_durations(&accumulate_lengths(&lengths), duration);

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
        let mut acc_elapsed = Duration::ZERO;

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
                    Some(SplineMap::from_bezier(&b0, &b1, &b2, &b3, true)),
                )
            },
        ))
    }

    pub fn with_track_ease(mut self, track_ease: Option<Ease>) -> Self {
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

    pub fn length(&self) -> f64 {
        self.intervals
            .iter()
            .fold(0.0, |acc, interval| acc + interval.length())
    }

    // Returns the sampled value at elapsed, as well as the values for any elapsed keyframes
    pub fn keyframe_sample(&self, elapsed: Duration) -> Vec<V> {
        std::iter::once(self.intervals[0].from)
            .chain(
                self.intervals
                    .iter()
                    .filter(move |interval| {
                        elapsed > interval.start
                            && !interval.changes_after(elapsed - interval.start)
                    })
                    .into_iter()
                    .map(|interval| interval.to),
            )
            .chain(std::iter::once(
                self.current_interval(&elapsed)
                    .map(|interval| interval.sample(elapsed))
                    .expect("Animation has no current interval!"),
            ))
            .collect()
    }
}

impl<V: Animatable> Animation<V> for IntervalTrack<V> {
    fn sample(&self, elapsed: Duration) -> V {
        let eased_elapsed = self
            .track_ease
            .as_ref()
            .map(|ease| {
                self.duration().mul_f64(ease.ease(
                    (elapsed - self.intervals[0].start).as_secs_f64()
                        / self.duration().as_secs_f64(),
                ))
            })
            .unwrap_or(elapsed);

        self.current_interval(&eased_elapsed)
            .expect("tried to sample empty `IntervalTrack`")
            .sample(eased_elapsed)
    }
}

impl<V: Animatable> BoundedAnimation<V> for IntervalTrack<V> {
    fn duration(&self) -> Duration {
        self.intervals
            .last()
            .map(|last| last.end)
            .unwrap_or_default()
    }
}

/// Different ways of selecting additional control points at either end of a series of values.
pub enum BookendStyle {
    /// Repeat the first and last values
    Repeat,
    /// Linearly extrapolate using the first two and last two values
    Linear,
    /// Use the starting/ending values to form a loop
    Loop,
    /// Use the first/last three points to calculate a point that would loop back toward the second-to-first/last point
    Spiral,
    /// Don't add additional bookends, as the user has already included external control points
    None,
}

fn bookend<V: Animatable>(values: Vec<V>, style: BookendStyle) -> Vec<V> {
    if values.is_empty() {
        values
    } else {
        match style {
            BookendStyle::Repeat => {
                let final_bookend = *values.last().unwrap();
                std::iter::once(values[0])
                    .chain(values)
                    .chain(std::iter::once(final_bookend))
                    .collect()
            }
            BookendStyle::Linear => {
                assert!(
                    values.len() >= 2,
                    "Linear bookending requires 2 or more values, but you only specified {}",
                    values.len()
                );
                let last_index = values.len() - 1;
                let initial_bookend = values[0].add(values[0].sub(values[1]));
                let final_bookend =
                    values[last_index].add(values[last_index].sub(values[last_index - 1]));

                std::iter::once(initial_bookend)
                    .chain(values)
                    .chain(std::iter::once(final_bookend))
                    .collect()
            }
            BookendStyle::Loop => {
                let initial_bookend = values[values.len() - 2];
                let final_bookend = values[1];

                std::iter::once(initial_bookend)
                    .chain(values)
                    .chain(std::iter::once(final_bookend))
                    .collect()
            }
            BookendStyle::Spiral => {
                assert!(
                    values.len() >= 3,
                    "Spiral bookending requires 3 or more values, but you only specified {}",
                    values.len()
                );
                let last_index = values.len() - 1;
                let initial_bookend = values[0].sub(values[1].sub(values[2]));
                let final_bookend =
                    values[last_index].sub(values[last_index - 1].sub(values[last_index - 2]));

                std::iter::once(initial_bookend)
                    .chain(values)
                    .chain(std::iter::once(final_bookend))
                    .collect()
            }
            BookendStyle::None => {
                assert!(
                    values.len() >= 4,
                    "Catmull-Rom Spline calculation requires four values, but you only specified {}. Perhaps you meant to use a Repeat or Linear BookendStyle?", values.len());
                values
            }
        }
    }
}

fn bookend_frames<V: Animatable>(frames: Vec<Frame<V>>, style: BookendStyle) -> Vec<Frame<V>> {
    let bookended_values = bookend(frames.iter().map(|frame| frame.value).collect(), style);
    let bookended_durations = std::iter::once(Duration::ZERO)
        .chain(frames.into_iter().map(|frame| frame.offset))
        .chain(std::iter::once(Duration::ZERO))
        .collect::<Vec<Duration>>();
    bookended_values
        .iter()
        .zip(bookended_durations)
        .map(|(v, d)| Frame::new(d, *v))
        .collect()
}

fn bookended_values_to_bezier_structs<V: Animatable>(
    values: &Vec<V>,
    rectify: bool,
) -> (Vec<BezierPath<V>>, Vec<SplineMap>) {
    values
        .windows(4)
        .map(|window| {
            let (b0, b1, b2, b3) =
                centripetal_catmull_rom_to_bezier(&window[0], &window[1], &window[2], &window[3]);
            (
                BezierPath::new(b1, b2),
                SplineMap::from_bezier(&b0, &b1, &b2, &b3, rectify),
            )
        })
        .unzip()
}

fn accumulate_lengths(lengths: &Vec<f64>) -> Vec<f64> {
    let mut accumulated_lengths = vec![];
    let total_length = lengths.iter().fold(0.0, |total, length| {
        accumulated_lengths.push(total);
        total + length
    });
    accumulated_lengths.push(total_length);

    accumulated_lengths
}

fn constant_velocity_durations(distances: &Vec<f64>, duration: Duration) -> Vec<Duration> {
    distances
        .iter()
        .map(|distance| duration.mul_f64(distance / distances.last().unwrap()))
        .collect()
}

impl<V: Animatable> Debug for IntervalTrack<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntervalTrack<{}>", std::any::type_name::<V>())
            .expect("Failed to write IntervalTrack type!");

        for interval in &self.intervals {
            write!(f, "\n  {:?}", interval).expect("Failed to print Interval information!");
        }
        write!(f, "\n\ttrack_ease:\t{:?}", self.track_ease)
    }
}
