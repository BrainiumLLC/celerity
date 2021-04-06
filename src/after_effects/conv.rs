use crate::Animatable;
use gee::en;

pub trait FromValue<C: en::Num = Self>: Animatable<C> {
    fn from_value(value: f64) -> Self;
}

pub trait FromMultiDimensional<C: en::Num = Self>: Animatable<C> {
    fn from_multi_dimensional(value: &[f64]) -> Option<Self>;
}

impl<T: en::Num> FromValue for T {
    fn from_value(value: f64) -> Self {
        // TODO: `en::try_cast`
        en::cast(value)
    }
}

impl FromValue<f64> for gee::Angle<f64> {
    fn from_value(value: f64) -> Self {
        Self::from_degrees(value)
    }
}

impl FromMultiDimensional<f64> for gee::Point<f64> {
    fn from_multi_dimensional(value: &[f64]) -> Option<Self> {
        (value.len() >= 2).then(|| {
            if value.len() > 2 {
                log::warn!(
                    "multidimensional value had more components than expected for `gee::Point`; ignoring extras"
                );
            }
            Self::new(value[0], value[1])
        })
    }
}

impl FromMultiDimensional<f64> for gee::Size<f64> {
    fn from_multi_dimensional(value: &[f64]) -> Option<Self> {
        (value.len() >= 2).then(|| {
            if value.len() > 2 {
                log::warn!(
                    "multidimensional value had more components than expected for `gee::Size`; ignoring extras"
                );
            }
            Self::new(value[0], value[1])
        })
    }
}

impl FromMultiDimensional<f64> for gee::Vector<f64> {
    fn from_multi_dimensional(value: &[f64]) -> Option<Self> {
        (value.len() >= 2).then(|| {
            if value.len() > 2 {
                log::warn!(
                    "multidimensional value had more components than expected for `gee::Vector`; ignoring extras"
                );
            }
            Self::new(value[0], value[1])
        })
    }
}

impl FromMultiDimensional<f64> for rainbow::LinRgba {
    fn from_multi_dimensional(value: &[f64]) -> Option<Self> {
        (value.len() == 4).then(|| {
            rainbow::SrgbRgba::from_f32(
                // TODO: should we care about the loss of precision?
                // (reminder: this will panic if we actually lose any)
                en::cast(value[0]),
                en::cast(value[1]),
                en::cast(value[2]),
                en::cast(value[3]),
            )
            .to_linear()
        })
    }
}
