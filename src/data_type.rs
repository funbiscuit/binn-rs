use crate::storage::Storage;
use crate::subtype::SubType;
use crate::Error;
use byteorder::{BigEndian, ByteOrder};

use crate::error::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Type {
    pub storage: Storage,
    pub subtype: SubType,
}

impl Type {
    /// Returns whether this type is represented as single byte or not
    pub fn is_u8(&self) -> bool {
        self.subtype.value() < 16
    }

    /// Returns how many bytes this type will take (1 or 2)
    pub fn size(&self) -> usize {
        if self.is_u8() {
            1
        } else {
            2
        }
    }

    /// Writes this type into buffer and returns new insert position
    ///
    /// # Panics:
    ///
    /// Panics if buffer is too short (less than 2 bytes)
    pub fn write<'a>(&self, buf: &'a mut [u8]) -> &'a mut [u8] {
        let storage = self.storage as u16;
        let subtype = self.subtype.value();

        if self.is_u8() {
            buf[0] = (storage | subtype) as u8;
            &mut buf[1..]
        } else {
            BigEndian::write_u16(buf, (storage << 8) | 0x1000 | subtype);
            &mut buf[2..]
        }
    }
}

impl TryFrom<&[u8]> for Type {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::Malformed);
        }

        let is_u8 = (value[0] & 0x10) == 0;

        // both storage and subtype
        let storage = (value[0] & 0xE0).try_into().unwrap();
        let subtype = if is_u8 {
            value[0] as u16 & 0x0F
        } else {
            if value.len() == 1 {
                return Err(Error::Malformed);
            }
            ((value[0] as u16 & 0x0F) << 8) | value[1] as u16
        }
        .try_into()
        .unwrap();

        Ok(Self { storage, subtype })
    }
}

pub const NULL: Type = Type {
    storage: Storage::NoBytes,
    subtype: SubType(0),
};

pub const TRUE: Type = Type {
    storage: Storage::NoBytes,
    subtype: SubType(1),
};

pub const FALSE: Type = Type {
    storage: Storage::NoBytes,
    subtype: SubType(2),
};

pub const UINT8: Type = Type {
    storage: Storage::Byte,
    subtype: SubType(0),
};

pub const INT8: Type = Type {
    storage: Storage::Byte,
    subtype: SubType(1),
};

pub const UINT16: Type = Type {
    storage: Storage::Word,
    subtype: SubType(0),
};

pub const INT16: Type = Type {
    storage: Storage::Word,
    subtype: SubType(1),
};

pub const UINT32: Type = Type {
    storage: Storage::DWord,
    subtype: SubType(0),
};

pub const INT32: Type = Type {
    storage: Storage::DWord,
    subtype: SubType(1),
};

pub const FLOAT: Type = Type {
    storage: Storage::DWord,
    subtype: SubType(2),
};

pub const UINT64: Type = Type {
    storage: Storage::QWord,
    subtype: SubType(0),
};

pub const INT64: Type = Type {
    storage: Storage::QWord,
    subtype: SubType(1),
};

pub const DOUBLE: Type = Type {
    storage: Storage::QWord,
    subtype: SubType(2),
};

pub const TEXT: Type = Type {
    storage: Storage::String,
    subtype: SubType(0),
};

pub const DATE_TIME: Type = Type {
    storage: Storage::String,
    subtype: SubType(1),
};

pub const DATE: Type = Type {
    storage: Storage::String,
    subtype: SubType(2),
};

pub const TIME: Type = Type {
    storage: Storage::String,
    subtype: SubType(3),
};

pub const DECIMAL_STR: Type = Type {
    storage: Storage::String,
    subtype: SubType(4),
};

pub const BLOB: Type = Type {
    storage: Storage::Blob,
    subtype: SubType(0),
};

pub const LIST: Type = Type {
    storage: Storage::Container,
    subtype: SubType(0),
};

pub const MAP: Type = Type {
    storage: Storage::Container,
    subtype: SubType(1),
};

pub const OBJECT: Type = Type {
    storage: Storage::Container,
    subtype: SubType(2),
};
