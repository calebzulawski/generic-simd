//! Types indicating widths of vectors.

/// Indicates the width of a vector.
pub trait Width {
    const VALUE: usize;
}

/// Indicates a vector contains 1 lane.
pub struct W1;

/// Indicates a vector contains 2 lanes.
pub struct W2;

/// Indicates a vector contains 4 lanes.
pub struct W4;

/// Indicates a vector contains 8 lanes.
pub struct W8;

impl Width for W1 {
    const VALUE: usize = 1;
}

impl Width for W2 {
    const VALUE: usize = 2;
}

impl Width for W4 {
    const VALUE: usize = 4;
}

impl Width for W8 {
    const VALUE: usize = 8;
}
