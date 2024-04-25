//! Defines setflags options.
//!
//! Since some operations in the Armv7-m and v6-m ISAs flag setting
//! behavior is dependent on wether or not the cpu is currently executing
//! in a conditional block we need to reflect this behavior in the disassembler.

/// Enumerates the possible SetFlags values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetFlags {
    /// Pre-determined.
    Literal(bool),
    /// Depends on wether or not the code is in an IT block or not.
    ///
    /// ```ignore
    /// let set_flags = !in_it_block ^ SetFlags::InitBlock(value)
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
/// let set_flags = !in_it_block ^ SetFlags::InitBlock(value)
/// ```
pub trait LocalUnwrap {
    /// Extracts the set flag option.
    ///
    /// If it depends on wether we are in an IT block or not
    /// we get the result of
    /// ```ignore
    /// let set_flags = !in_it_block ^ SetFlags::InitBlock(value)
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

#[cfg(test)]
mod test {
    use super::LocalUnwrap;
    use crate::SetFlags;

    #[test]
    fn test_unwrap() {
        let set_flags = Some(SetFlags::Literal(false));
        assert!(!set_flags.local_unwrap(false));
        let set_flags = Some(SetFlags::Literal(false));
        assert!(!set_flags.local_unwrap(true));
        let set_flags = Some(SetFlags::Literal(true));
        assert!(set_flags.local_unwrap(true));
        let set_flags = None;
        assert!(!set_flags.local_unwrap(false));

        let set_flags = None;
        assert!(!set_flags.local_unwrap(false));

        let set_flags = Some(SetFlags::InITBlock(false));
        assert!(set_flags.local_unwrap(false));
        let set_flags = Some(SetFlags::InITBlock(true));
        assert!(!set_flags.local_unwrap(false));
        let set_flags = Some(SetFlags::InITBlock(false));
        assert!(!set_flags.local_unwrap(true));
        let set_flags = Some(SetFlags::InITBlock(true));
        assert!(set_flags.local_unwrap(true));
    }
}
