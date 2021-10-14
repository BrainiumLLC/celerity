use gee::en;

/// A value that has zero or more numeric components.
pub trait ComponentWise: Sized {
    type Component: en::Num;

    /// Runs `f` on each component, replacing the component with the return
    /// value.
    ///
    /// Not truly map, since it can't change the component type.
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self::Component) -> Self::Component;

    /// Runs `f` on each component paired with the corresponding component from
    /// `other`, replacing the component with the return value.
    ///
    /// This is the basic building block for generic component-wise operations.
    fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(Self::Component, Self::Component) -> Self::Component;

    /// Component-wise addition.
    fn add(self, other: Self) -> Self {
        self.zip_map(other, std::ops::Add::add)
    }

    /// Component-wise subtraction.
    fn sub(self, other: Self) -> Self {
        self.zip_map(other, std::ops::Sub::sub)
    }

    /// Convenience for casting a number to the component type.
    fn cast_component<T: en::Num>(other: T) -> Self::Component {
        en::cast(other)
    }
}
