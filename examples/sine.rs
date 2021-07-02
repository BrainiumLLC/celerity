use celerity::{function::Function, Animation as _};
use time_point::{Duration, TimePoint};

fn sine(elapsed: Duration) -> f32 {
    elapsed.as_secs_f32().recip().sin()
}

fn now() -> TimePoint {
    TimePoint::from_std_instant(std::time::Instant::now())
}

fn main() {
    let anim = Function::new(sine);
    let start = now();
    for _ in 0..100 {
        println!("{}", anim.sample(now() - start));
    }
}
