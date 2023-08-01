pub type Result<T> = core::result::Result<T, Error>;

/// Error that might occur when using binn values
#[derive(Debug)]
pub enum Error {
    /// Attempted to insert value with key longer than 255 bytes
    LongKey,

    /// Given byte buffer was malformed and couldn't be parsed
    Malformed,

    /// Container was read only and cannot be modified
    ReadOnly,

    /// Indicates that static buffer was not big enough and contains
    /// how many extra bytes are needed
    SmallBuffer(usize),
}

#[derive(Debug)]
pub struct OutOfRangeError;
