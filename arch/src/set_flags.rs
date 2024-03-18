//! Defines setflags options.

/// Enumerates the possible SetFlags values
#[derive(Debug,Clone,Copy,PartialEq)]
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

