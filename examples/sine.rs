use celerity::{function::Function, Animation as _};
use std::time::{Duration, Instant};

fn sine(elapsed: Duration) -> f32 {
    elapsed.as_secs_f32().recip().sin()
}

fn main() {
    let anim = Function::new(sine);
    let start = Instant::now();
    for _ in 0..100 {
        println!("{}", anim.sample(Instant::now() - start));
    }
}
