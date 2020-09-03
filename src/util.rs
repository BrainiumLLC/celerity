use time_point::Duration;

pub trait Map<T>: Sized {
    fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(T) -> T;
}

pub trait ZipMap<T>: Map<T> {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(T, T) -> T;
}

impl Map<i64> for Duration {
    fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(i64) -> i64,
    {
        Duration::new(f(self.nanos))
    }
}

impl ZipMap<i64> for Duration {
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(i64, i64) -> i64,
    {
        self.map(|nanos| f(nanos, other.nanos))
    }
}
