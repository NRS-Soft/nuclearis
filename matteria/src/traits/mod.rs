
/// Custom trait for ASCII charcodes operations
pub trait ASCII {
    fn to_ascii(&self) -> Result<u8, ()> {
        Err(())
    }
}

/// ASCII trait implementations
impl ASCII for u8 {
    /// Returns a Result after trying add 48 to a value
    fn to_ascii(&self) -> Result<u8, ()> {
        match &self.checked_add(48u8) {
            Some(value) => Ok(*value as u8),
            None => Err(()),
        }
    }
}
