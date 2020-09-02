use time_point::Duration;

macro_rules! assert_start_lte_time {
    ($name:ident, $start:expr, $time:expr) => {
        debug_assert!(
            $start <= $time,
            concat!(
                stringify!($name),
                " sampled with a start later than the time:\nstart: {:?}\ntime : {:?}"
            ),
            $start.nanos_since_zero,
            $time.nanos_since_zero
        );
    };
}

pub fn multiply_duration(duration: Duration, i: usize) -> Duration {
    (0..i).fold(Duration::zero(), |total, _| total + duration)
}
