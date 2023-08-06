use crate::error::{DeserializeError, OutOfRangeError};
use crate::utils;
use byteorder::{BigEndian, ByteOrder};

/// Maximum possible size that can be used
pub const MAX_SIZE: u32 = 0x7FFFFFFF;
pub const MAX_SIZE_MASK: u32 = 0x80000000;

/// Size can be either in compact form (if it is less than 127)
/// and than it takes only 1 byte, or in full form (otherwise)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Size {
    Compact(u8),
    Full(u32),
}

impl Size {
    pub fn is_compactable(value: usize) -> bool {
        value <= 127
    }

    pub fn is_u8(&self) -> bool {
        self.size() == 1
    }

    pub fn new(value: usize) -> Result<Self, OutOfRangeError<usize>> {
        if Self::is_compactable(value) {
            Ok(Size::Compact(value as u8))
        } else if value <= MAX_SIZE as usize {
            Ok(Size::Full(value as u32))
        } else {
            Err(OutOfRangeError {
                min: Some(0),
                max: Some(MAX_SIZE as usize),
                value,
            })
        }
    }

    /// How many bytes this size will take to serialize
    pub fn size(&self) -> usize {
        match self {
            Size::Compact(_) => 1,
            Size::Full(_) => 4,
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Size::Compact(v) => *v as usize,
            Size::Full(v) => *v as usize,
        }
    }

    /// Writes this size to given buffer in its compact or full form
    /// and returns next insert position
    ///
    /// # Panics
    ///
    /// Panics if given buffer is not big enough
    pub fn write<'b>(&self, buf: &'b mut [u8]) -> &'b mut [u8] {
        let total_size = self.size();

        assert!(total_size <= buf.len());

        match self {
            Size::Compact(v) => {
                buf[0] = *v;
            }
            Size::Full(v) => {
                BigEndian::write_u32(buf, *v | MAX_SIZE_MASK);
            }
        }

        &mut buf[total_size..]
    }
}

impl<'a> TryFrom<&'a [u8]> for Size {
    type Error = DeserializeError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            return Err(DeserializeError::InvalidData);
        }

        if (bytes[0] & 0x80) == 0 {
            // compact form is used
            Ok(Size::Compact(bytes[0]))
        } else if let Ok(v) = utils::read_u32(bytes) {
            // remove first bit
            Ok(Size::Full(v & MAX_SIZE))
        } else {
            Err(DeserializeError::InvalidData)
        }
    }
}
