use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A handle for a CPU feature.
pub trait Feature: Sized {
    /// Create a new CPU feature handle, returning None if the feature is not supported by the
    /// processor.
    fn new() -> Option<Self>;

    /// Create a new CPU feature handle without checking if the feature is supported.
    unsafe fn new_unchecked() -> Self;
}

/// A handle for a specific vector type.
pub trait Capability<Scalar>: Feature {
    /// The supported vector type.
    type Vector: Vector<Scalar = Scalar>;

    /// Load a vector from a pointer.
    unsafe fn load_ptr(&self, from: *const Scalar) -> Self::Vector;

    /// Load a vector from a slice without checking the length.
    unsafe fn load_unchecked<T: AsRef<[Scalar]>>(&self, from: T) -> Self::Vector {
        self.load_ptr(from.as_ref().as_ptr())
    }

    /// Load a vector from a slice.  Panics if the slice is not long enough.
    fn load<T: AsRef<[Scalar]>>(&self, from: T) -> Self::Vector {
        assert!(
            from.as_ref().len() >= Self::Vector::width(),
            "source not large enough to load vector"
        );
        unsafe { self.load_unchecked(from) }
    }

    /// Broadcast a value into each lane of the vector.
    fn splat(&self, from: Scalar) -> Self::Vector;

    /// Create a new vector containing all zeros.
    fn zero(&self) -> Self::Vector;
}

/// The fundamental vector type.
pub unsafe trait VectorCore: Sized {
    /// The type of elements in the vector.
    type Scalar;

    /// Returns the number of elements in the vector.
    fn width() -> usize {
        std::mem::size_of::<Self>() / std::mem::size_of::<Self::Scalar>()
    }

    /// Returns a slice containing the vector.
    fn as_slice(&self) -> &[Self::Scalar] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const _, Self::width()) }
    }

    /// Returns a mutable slice containing the vector.
    fn as_slice_mut(&mut self) -> &mut [Self::Scalar] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut _ as *mut _, Self::width()) }
    }

    /// Store to a pointer.
    unsafe fn store_ptr(self, to: *mut Self::Scalar);

    /// Store to a slice without checking the length.
    unsafe fn store_unchecked<T: AsMut<[Self::Scalar]>>(self, mut to: T) {
        self.store_ptr(to.as_mut().as_mut_ptr());
    }

    /// Store to a slice.
    fn store<T: AsMut<[Self::Scalar]>>(self, mut to: T) {
        assert!(
            to.as_mut().len() >= Self::width(),
            "destination not large enough to store vector"
        );
        unsafe { self.store_unchecked(to) };
    }
}

pub trait Vector:
    VectorCore
    + Add<Self>
    + AddAssign<Self>
    + Sub<Self>
    + SubAssign<Self>
    + Mul<Self>
    + MulAssign<Self>
    + Div<Self>
    + DivAssign<Self>
    + Neg
{
    type Scalar;
}
impl<S, V> Vector for V
where
    V: VectorCore<Scalar = S>
        + Add<Self>
        + AddAssign<Self>
        + Sub<Self>
        + SubAssign<Self>
        + Mul<Self>
        + MulAssign<Self>
        + Div<Self>
        + DivAssign<Self>
        + Neg,
{
    type Scalar = S;
}
