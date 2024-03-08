//! Defines architechture specific helpers, these are not used in this crate
//! but are provided as a helper for users of this crate and as a supplement for
//! documentation.

/// Application status register.
pub struct Apsr {
    /// true if the result is negative.
    pub n: bool,
    /// true if the result is zero.
    pub z: bool,
    /// true if the operation resulted in carry.
    pub c: bool,
    /// true if the operation resulted in overflow.
    pub v: bool,
    /// true if the operation changes the input value.
    pub q: bool,
    /// Greater than or equal flags used for SEL.
    pub ge: [[bool; 4]; 2],
}
