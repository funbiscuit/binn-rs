/// Error type for add value operation
#[derive(Debug)]
pub enum AddValueError {
    /// Given key (when inserting value into object) length is longer than 255 bytes
    LongKey,

    /// Container is not mutable and cannot be modified
    ReadOnly,

    /// Static buffer of container was not big enough to add new value
    SmallBuffer(SmallBufferError),
}

/// Error type for binn value deserialization
#[derive(Debug)]
pub enum DeserializeError {
    /// \[data\] block was invalid, or \[size\] and \[data\] blocks were invalid
    InvalidData,

    /// Type of value was invalid
    InvalidType,
}

/// Error type for various convert operations. Error occurs when provided value
/// is not within required range. Contains valid range of values and value, that was given.
#[derive(Debug)]
pub struct OutOfRangeError<T> {
    /// Lower bound of required range. If None, then no lower bound present
    pub min: Option<T>,

    /// Upper bound of required range. If None, then no upper bound present
    pub max: Option<T>,

    /// Given value, that lies out of required range
    pub value: T,
}

/// Error type that indicates insufficient buffer size
#[derive(Debug)]
pub struct SmallBufferError {
    /// How many bytes must be added to buffer for operation to succeed
    pub required_extra: usize,
}

impl From<SmallBufferError> for AddValueError {
    fn from(value: SmallBufferError) -> Self {
        AddValueError::SmallBuffer(value)
    }
}
