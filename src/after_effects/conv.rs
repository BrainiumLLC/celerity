use crate::Animatable;
use gee::en;
use thiserror::Error;

pub trait FromValue<C: en::Num = Self>: Animatable<C> {
    type Error: std::error::Error;

    fn from_value(value: f64) -> Result<Self, Self::Error>;
}

pub trait FromMultiDimensional<C: en::Num = Self>: Animatable<C> {
    type Error: std::error::Error;

    fn from_multi_dimensional(value: &[f64]) -> Result<Self, Self::Error>;
}

impl<T: en::Num> FromValue for T {
    type Error = en::CastFailure<T, f64>;

    fn from_value(value: f64) -> Result<Self, Self::Error> {
        en::try_cast(value)
    }
}

impl FromValue<f64> for gee::Angle<f64> {
    type Error = std::convert::Infallible;

    fn from_value(value: f64) -> Result<Self, Self::Error> {
        Ok(Self::from_degrees(value))
    }
}

pub struct WrongLen<T> {
    expected: usize,
    found: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for WrongLen<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrongLen")
            .field("expected", &self.expected)
            .field("found", &self.found)
            .finish()
    }
}

impl<T> std::fmt::Display for WrongLen<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.expected != self.found {
            write!(
                f,
                "type `{}` expected {} components, but multidimensional value contained {} components",
                std::any::type_name::<T>(),
                self.expected,
                self.found,
            )
        } else {
            unreachable!("`WrongLen` error produced despite `expected` and `found` matching")
        }
    }
}

impl<T> std::error::Error for WrongLen<T> {}

impl<T> WrongLen<T> {
    fn check<U>(value: &[f64], expected: usize, f: impl FnOnce(&[f64]) -> U) -> Result<U, Self> {
        let found = value.len();
        (found == expected).then(|| f(value)).ok_or_else(|| Self {
            expected,
            found,
            _marker: std::marker::PhantomData,
        })
    }
}

#[derive(Debug, Error)]
pub enum CastFailedOrWrongLen<V: std::fmt::Debug + 'static, C: std::fmt::Debug + 'static> {
    #[error(transparent)]
    CastFailed(#[from] en::CastFailure<C, f64>),
    #[error(transparent)]
    WrongLen(#[from] WrongLen<V>),
}

impl FromMultiDimensional<f64> for gee::Point<f64> {
    type Error = WrongLen<Self>;

    fn from_multi_dimensional(value: &[f64]) -> Result<Self, Self::Error> {
        WrongLen::check(value, 2, |value| Self::new(value[0], value[1]))
    }
}

impl FromMultiDimensional<f64> for gee::Size<f64> {
    type Error = WrongLen<Self>;

    fn from_multi_dimensional(value: &[f64]) -> Result<Self, Self::Error> {
        WrongLen::check(value, 2, |value| Self::new(value[0], value[1]))
    }
}

impl FromMultiDimensional<f64> for gee::Vector<f64> {
    type Error = WrongLen<Self>;

    fn from_multi_dimensional(value: &[f64]) -> Result<Self, Self::Error> {
        // TODO: ignore z in a more deliberate/explicit fashion
        WrongLen::check(value, 3, |value| Self::new(value[0], value[1])).or_else(
            |Self::Error { .. }| WrongLen::check(value, 2, |value| Self::new(value[0], value[1])),
        )
    }
}

impl FromMultiDimensional<f64> for rainbow::LinRgba {
    type Error = CastFailedOrWrongLen<Self, f32>;

    fn from_multi_dimensional(value: &[f64]) -> Result<Self, Self::Error> {
        WrongLen::<Self>::check(value, 4, |value| {
            Ok(rainbow::SrgbRgba::from_f32(
                // TODO: should we care about the loss of precision?
                // (reminder: this will panic if we actually lose any)
                en::try_cast(value[0])?,
                en::try_cast(value[1])?,
                en::try_cast(value[2])?,
                en::try_cast(value[3])?,
            )
            .to_linear())
        })
        .map_err(Self::Error::from)?
    }
}
