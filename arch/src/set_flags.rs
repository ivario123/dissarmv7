//! Defines setflags options.

/// Enumerates the possible SetFlags values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetFlags {
    /// Pre-determined.
    Literal(bool),
    /// Depends on wether or not the code is in an IT block or not.
    ///
    /// ```ignore
    /// let set_flags = !in_it_block ^ SetFlags::InitBlock(true)
    /// ```
    InITBlock(bool),
}

impl From<bool> for SetFlags {
    fn from(value: bool) -> Self {
        Self::Literal(value)
    }
}

/// Extracts the set flag option.
///
/// If it depends on wether we are in an IT block or not
/// we get the result of
/// ```ignore
/// let set_flags = !in_it_block ^ SetFlags::InitBlock(true)
/// ```
pub trait LocalUnwrap {
    /// Extracts the set flag option.
    ///
    /// If it depends on wether we are in an IT block or not
    /// we get the result of
    /// ```ignore
    /// let set_flags = !in_it_block ^ SetFlags::InitBlock(true)
    /// ```
    fn local_unwrap(self, in_it_block: bool) -> bool
    where
        Self: Sized;
}
impl LocalUnwrap for Option<SetFlags> {
    fn local_unwrap(self, in_it_block: bool) -> bool {
        match self {
            Some(SetFlags::Literal(b)) => b,
            Some(SetFlags::InITBlock(b)) => (!in_it_block) ^ b,
            None => false,
        }
    }
}
