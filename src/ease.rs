use crate::spline::bezier_ease::BezierEase;
use std::fmt::Debug;

pub type EaseFunction = fn(f64) -> f64;

#[derive(Clone, Copy)]
pub enum Ease {
    Bezier(BezierEase),
    Function(EaseFunction),
}

impl Debug for Ease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ease::Bezier(ease) => write!(f, "Ease::Bezier({:?})", ease),
            Ease::Function(_) => write!(f, "Ease::Function(...)"),
        }
    }
}

impl Ease {
    pub fn ease(&self, t: f64) -> f64 {
        match self {
            Ease::Bezier(bezier) => bezier.ease(t),
            Ease::Function(ease) => ease(t),
        }
    }

    pub fn in_sine() -> Self {
        Self::Function(in_sine)
    }
    pub fn out_sine() -> Self {
        Self::Function(out_sine)
    }
    pub fn in_out_sine() -> Self {
        Self::Function(in_out_sine)
    }

    pub fn in_quad() -> Self {
        Self::Function(in_quad)
    }
    pub fn out_quad() -> Self {
        Self::Function(out_quad)
    }
    pub fn in_out_quad() -> Self {
        Self::Function(in_out_quad)
    }

    pub fn in_cubic() -> Self {
        Self::Function(in_cubic)
    }
    pub fn out_cubic() -> Self {
        Self::Function(out_cubic)
    }
    pub fn in_out_cubic() -> Self {
        Self::Function(in_out_cubic)
    }

    pub fn in_quart() -> Self {
        Self::Function(in_quart)
    }
    pub fn out_quart() -> Self {
        Self::Function(out_quart)
    }
    pub fn in_out_quart() -> Self {
        Self::Function(in_out_quart)
    }

    pub fn in_quint() -> Self {
        Self::Function(in_quint)
    }
    pub fn out_quint() -> Self {
        Self::Function(out_quint)
    }
    pub fn in_out_quint() -> Self {
        Self::Function(in_out_quint)
    }

    pub fn in_expo() -> Self {
        Self::Function(in_expo)
    }
    pub fn out_expo() -> Self {
        Self::Function(out_expo)
    }
    pub fn in_out_expo() -> Self {
        Self::Function(in_out_expo)
    }

    pub fn in_circle() -> Self {
        Self::Function(in_circle)
    }
    pub fn out_circle() -> Self {
        Self::Function(out_circle)
    }
    pub fn in_out_circle() -> Self {
        Self::Function(in_out_circle)
    }

    pub fn in_back() -> Self {
        Self::Function(in_back)
    }
    pub fn out_back() -> Self {
        Self::Function(out_back)
    }
    pub fn in_out_back() -> Self {
        Self::Function(in_out_back)
    }

    pub fn in_elastic() -> Self {
        Self::Function(in_elastic)
    }
    pub fn out_elastic() -> Self {
        Self::Function(out_elastic)
    }
    pub fn in_out_elastic() -> Self {
        Self::Function(in_out_elastic)
    }

    pub fn in_bounce() -> Self {
        Self::Function(in_bounce)
    }
    pub fn out_bounce() -> Self {
        Self::Function(out_bounce)
    }
    pub fn in_out_bounce() -> Self {
        Self::Function(in_out_bounce)
    }

    pub fn none() -> Self {
        Self::Function(identity)
    }
    pub fn lazy() -> Self {
        Self::Function(lazy)
    }
}

// All of the eases shown here are included for your convenience: https://easings.net/

// Sine Eases =================================================================

fn in_sine(t: f64) -> f64 {
    1.0 - f64::cos(t * std::f64::consts::FRAC_PI_2)
}

fn out_sine(t: f64) -> f64 {
    f64::sin(t * std::f64::consts::FRAC_PI_2)
}

fn in_out_sine(t: f64) -> f64 {
    -(f64::cos(t * std::f64::consts::PI) - 1.0) / 2.0
}

// Exponential Eases ==========================================================

fn in_exponential(t: f64, n: i32) -> f64 {
    t.powi(n)
}

fn out_exponential(t: f64, n: i32) -> f64 {
    1.0 - in_exponential(t, n)
}

fn in_out_exponential(t: f64, n: i32) -> f64 {
    (t < 0.5)
        .then(|| f64::powi(2.0, n - 1) * in_exponential(t, n))
        .unwrap_or_else(|| 1.0 - (-2.0 * t + 2.0).powi(n) / 2.0)
}

// Quad Eases =================================================================

fn in_quad(t: f64) -> f64 {
    in_exponential(t, 2)
}

fn out_quad(t: f64) -> f64 {
    out_exponential(t, 2)
}

fn in_out_quad(t: f64) -> f64 {
    in_out_exponential(t, 2)
}

// Cubic Eases ================================================================

fn in_cubic(t: f64) -> f64 {
    in_exponential(t, 3)
}

fn out_cubic(t: f64) -> f64 {
    out_exponential(t, 3)
}

fn in_out_cubic(t: f64) -> f64 {
    in_out_exponential(t, 3)
}

// Quart Eases ================================================================

fn in_quart(t: f64) -> f64 {
    in_exponential(t, 4)
}

fn out_quart(t: f64) -> f64 {
    out_exponential(t, 4)
}

fn in_out_quart(t: f64) -> f64 {
    in_out_exponential(t, 4)
}

// Quint Eases =================================================================

fn in_quint(t: f64) -> f64 {
    in_exponential(t, 5)
}

fn out_quint(t: f64) -> f64 {
    out_exponential(t, 5)
}

fn in_out_quint(t: f64) -> f64 {
    in_out_exponential(t, 5)
}

// Expo Eases =================================================================

fn in_expo(t: f64) -> f64 {
    2.0_f64.powf(10.0 * t - 10.0)
}

fn out_expo(t: f64) -> f64 {
    2.0_f64.powf(-10.0 * t)
}

fn in_out_expo(t: f64) -> f64 {
    (t == 0.0 || t == 1.0).then(|| t).unwrap_or_else(|| {
        if t < 0.5 {
            2.0_f64.powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
        }
    })
}

// Circle Eases ===============================================================

fn in_circle(t: f64) -> f64 {
    1.0 - (1.0 - t.powi(2)).sqrt()
}

fn out_circle(t: f64) -> f64 {
    (1.0 - t.powi(2)).sqrt()
}

fn in_out_circle(t: f64) -> f64 {
    (t < 0.5)
        .then(|| (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0)
        .unwrap_or_else(|| ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0)
}

// Back Eases =================================================================

fn in_back(t: f64) -> f64 {
    2.70158 * t.powi(3) - 1.70158 * t.powi(2)
}

fn out_back(t: f64) -> f64 {
    1.0 + 2.70158 * (t - 1.0).powi(3) + 1.70158 * (t - 1.0).powi(2)
}

fn in_out_back(t: f64) -> f64 {
    let c = 2.5949095;

    (t < 0.5)
        .then(|| ((t * 2.0).powi(2) * ((c + 1.0) * 2.0 * t - c)) / 2.0)
        .unwrap_or_else(|| {
            ((2.0 * t - 2.0).powi(2) * ((c + 1.0) * (t * 2.0 - 2.0) + c) + 2.0) / 2.0
        })
}

// Elastic Eases ==============================================================

fn in_elastic(t: f64) -> f64 {
    let c = std::f64::consts::TAU / 3.0;
    (t == 0.0 || t == 1.0)
        .then(|| t)
        .unwrap_or_else(|| -f64::powf(2.0, 10.0 * t - 10.0) * f64::sin((10.0 * t - 10.75) * c))
}

fn out_elastic(t: f64) -> f64 {
    let c = std::f64::consts::TAU / 3.0;
    (t == 0.0 || t == 1.0)
        .then(|| t)
        .unwrap_or_else(|| -f64::powf(2.0, t * -10.0) * f64::sin((10.0 * t - 0.75) * c))
}

fn in_out_elastic(t: f64) -> f64 {
    let c = std::f64::consts::TAU / 4.5;
    (t == 0.0 || t == 1.0).then(|| t).unwrap_or_else(|| {
        if t < 0.5 {
            (-f64::powf(2.0, 20.0 * t - 10.0) * f64::sin((20.0 * t - 11.125) * c)) / 2.0
        } else {
            (f64::powf(2.0, -20.0 * t + 10.0) * f64::sin((20.0 * t - 11.125) * c)) / 2.0 + 1.0
        }
    })
}

// Bounce Eases ===============================================================

fn in_bounce(t: f64) -> f64 {
    1.0 - out_bounce(1.0 - t)
}

fn out_bounce(t: f64) -> f64 {
    let n = 7.5625;
    let d = 2.75;

    if t < 1.0 / d {
        n * t.powi(2)
    } else if t < 2.0 / d {
        n * (t - (1.5 / d)) * (t - (1.5 / d)) + 0.75
    } else if t < 2.5 / d {
        n * (t - (2.25 / d)) * (t - (2.25 / d)) + 0.9375
    } else {
        n * (t - (2.625 / d)) * (t - (2.625 / d)) + 0.984375
    }
}

fn in_out_bounce(t: f64) -> f64 {
    (t < 0.5)
        .then(|| (1.0 - out_bounce(1.0 - 2.0 * t)) / 2.0)
        .unwrap_or_else(|| (1.0 + out_bounce(2.0 * t - 1.0)) / 2.0)
}

// Miscellaneous ==============================================================

fn identity(t: f64) -> f64 {
    t
}

fn lazy(t: f64) -> f64 {
    (t < 0.5).then(|| 0.0).unwrap_or_else(|| 2.0 * (t - 0.5))
}
