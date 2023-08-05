use crate::data_type::Type;
use crate::storage::Storage;
use crate::subtype::SubType;
use crate::{data_type, utils, Error, List, Map, Object};
use byteorder::{BigEndian, ByteOrder};

use crate::error::Result;
use crate::raw_container::{KeyType, RawContainer};
use crate::size::Size;

/// Any value in binn format
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    /// Null
    Null,

    /// Boolean True
    True,

    /// Boolean False
    False,

    /// Unsigned 8bit integer (0..255)
    UInt8(u8),

    /// Signed 8bit integer (-128..127)
    Int8(i8),

    /// Unsigned 16bit integer (0..65_535)
    UInt16(u16),

    /// Signed 16bit integer (-32_768..32_767)
    Int16(i16),

    /// Unsigned 32bit integer (0..4_294_967_295)
    UInt32(u32),

    /// Signed 32bit integer (-2_147_483_648..2_147_483_647)
    Int32(i32),

    /// IEEE 754 single precision floating point number (32bit)
    Float(f32),

    /// Unsigned 64bit integer (0..18_446_744_073_709_551_615)
    UInt64(u64),

    /// Signed 64bit integer (-9_223_372_036_854_775_808..9_223_372_036_854_775_807)
    Int64(i64),

    /// IEEE 754 double precision floating point number (64bit)
    Double(f64),

    /// UTF-8 encoded string
    Text(&'a str),

    /// String representing datetime (exact format not specified)
    DateTime(&'a str),

    /// String representing date (exact format not specified)
    Date(&'a str),

    /// String representing time (exact format not specified)
    Time(&'a str),

    /// String representing decimal number (exact format not specified)
    DecimalStr(&'a str),

    /// Binary data
    Blob(&'a [u8]),

    /// Container that stores elements sequentially without keys
    List(List<'a>),

    /// Container that stores key-value pairs with 32bit signed integer as keys
    Map(Map<'a>),

    /// Container that stores key-value pairs with utf-8 strings as keys
    /// (with 255 byte limit for their length)
    Object(Object<'a>),

    /// User-defined type with empty storage
    Empty(SubType),

    /// User-defined type with Byte storage (8bits)
    Byte(SubType, u8),

    /// User-defined type with Word storage (16bits)
    Word(SubType, u16),

    /// User-defined type with DWord storage (32bits)
    DWord(SubType, u32),

    /// User-defined type with QWord storage (64bits)
    QWord(SubType, u64),

    /// User-defined type with Text storage (UTF-8 string)
    UserText(SubType, &'a str),

    /// User-defined type with Blob storage (binary data)
    UserBlob(SubType, &'a [u8]),
}

impl<'a> Value<'a> {
    /// Try to deserialize given bytes as binn value
    pub fn deserialize(bytes: &'a [u8]) -> Result<Self> {
        let data_type: Type = bytes.try_into()?;
        let value = &bytes[data_type.size()..];

        match data_type {
            data_type::NULL => return Ok(Value::Null),
            data_type::TRUE => return Ok(Value::True),
            data_type::FALSE => return Ok(Value::False),
            data_type::UINT8 => return Ok(Value::UInt8(utils::read_u8(value)?)),
            data_type::INT8 => return Ok(Value::Int8(utils::read_i8(value)?)),
            data_type::UINT16 => return Ok(Value::UInt16(utils::read_u16(value)?)),
            data_type::INT16 => return Ok(Value::Int16(utils::read_i16(value)?)),
            data_type::UINT32 => return Ok(Value::UInt32(utils::read_u32(value)?)),
            data_type::INT32 => return Ok(Value::Int32(utils::read_i32(value)?)),
            data_type::FLOAT => return Ok(Value::Float(utils::read_f32(value)?)),
            data_type::UINT64 => return Ok(Value::UInt64(utils::read_u64(value)?)),
            data_type::INT64 => return Ok(Value::Int64(utils::read_i64(value)?)),
            data_type::DOUBLE => return Ok(Value::Double(utils::read_f64(value)?)),
            data_type::TEXT => return Ok(Value::Text(utils::read_text(value)?)),
            data_type::DATE_TIME => return Ok(Value::DateTime(utils::read_text(value)?)),
            data_type::DATE => return Ok(Value::Date(utils::read_text(value)?)),
            data_type::TIME => return Ok(Value::Time(utils::read_text(value)?)),
            data_type::DECIMAL_STR => return Ok(Value::DecimalStr(utils::read_text(value)?)),
            data_type::BLOB => return Ok(Value::Blob(utils::read_blob(value)?)),
            Type {
                storage: Storage::NoBytes,
                subtype,
            } => return Ok(Value::Empty(subtype)),
            Type {
                storage: Storage::Byte,
                subtype,
            } => return Ok(Value::Byte(subtype, utils::read_u8(value)?)),
            Type {
                storage: Storage::Word,
                subtype,
            } => return Ok(Value::Word(subtype, utils::read_u16(value)?)),
            Type {
                storage: Storage::DWord,
                subtype,
            } => return Ok(Value::DWord(subtype, utils::read_u32(value)?)),
            Type {
                storage: Storage::QWord,
                subtype,
            } => return Ok(Value::QWord(subtype, utils::read_u64(value)?)),
            Type {
                storage: Storage::String,
                subtype,
            } => return Ok(Value::UserText(subtype, utils::read_text(value)?)),
            Type {
                storage: Storage::Blob,
                subtype,
            } => return Ok(Value::UserBlob(subtype, utils::read_blob(value)?)),
            // container storage is handled separately
            Type {
                storage: Storage::Container,
                subtype: _,
            } => {}
        }
        // all simple values handled, now deserialize container

        match data_type {
            data_type::LIST => Ok(Value::List(List {
                inner: RawContainer::from_bytes(bytes, KeyType::Empty)?,
            })),
            data_type::MAP => Ok(Value::Map(Map {
                inner: RawContainer::from_bytes(bytes, KeyType::Num)?,
            })),
            data_type::OBJECT => Ok(Value::Object(Object {
                inner: RawContainer::from_bytes(bytes, KeyType::Str)?,
            })),
            Type {
                storage: Storage::Container,
                subtype: _,
            } => Err(Error::Malformed),
            _ => unreachable!(),
        }
    }

    /// Returns how many bytes \[data\] will take, when it needs \[size\] element
    ///
    /// # Panics
    ///
    /// Panics if value is container
    pub(crate) fn data_size(&self) -> Option<usize> {
        match self {
            Value::Text(t)
            | Value::DateTime(t)
            | Value::Date(t)
            | Value::Time(t)
            | Value::DecimalStr(t)
            | Value::UserText(_, t) => Some(t.len() + 1),

            Value::Blob(b) | Value::UserBlob(_, b) => Some(b.len()),

            Value::List(_) | Value::Map(_) | Value::Object(_) => unreachable!(),

            _ => None,
        }
    }

    /// Returns type of this value (subtype, storage)
    pub(crate) fn get_type(&self) -> Type {
        match self {
            Value::Null => data_type::NULL,
            Value::True => data_type::TRUE,
            Value::False => data_type::FALSE,
            Value::UInt8(_) => data_type::UINT8,
            Value::Int8(_) => data_type::INT8,
            Value::UInt16(_) => data_type::UINT16,
            Value::Int16(_) => data_type::INT16,
            Value::UInt32(_) => data_type::UINT32,
            Value::Int32(_) => data_type::INT32,
            Value::Float(_) => data_type::FLOAT,
            Value::UInt64(_) => data_type::UINT64,
            Value::Int64(_) => data_type::INT64,
            Value::Double(_) => data_type::DOUBLE,
            Value::Text(_) => data_type::TEXT,
            Value::DateTime(_) => data_type::DATE_TIME,
            Value::Date(_) => data_type::DATE,
            Value::Time(_) => data_type::TIME,
            Value::DecimalStr(_) => data_type::DECIMAL_STR,
            Value::Blob(_) => data_type::BLOB,
            Value::List(_) => data_type::LIST,
            Value::Map(_) => data_type::MAP,
            Value::Object(_) => data_type::OBJECT,
            Value::Empty(sub) => Type {
                storage: Storage::NoBytes,
                subtype: *sub,
            },
            Value::Byte(sub, _) => Type {
                storage: Storage::Byte,
                subtype: *sub,
            },
            Value::Word(sub, _) => Type {
                storage: Storage::Word,
                subtype: *sub,
            },
            Value::DWord(sub, _) => Type {
                storage: Storage::DWord,
                subtype: *sub,
            },
            Value::QWord(sub, _) => Type {
                storage: Storage::QWord,
                subtype: *sub,
            },
            Value::UserText(sub, _) => Type {
                storage: Storage::String,
                subtype: *sub,
            },
            Value::UserBlob(sub, _) => Type {
                storage: Storage::Blob,
                subtype: *sub,
            },
        }
    }

    /// Returns how many bytes this value will take in buffer
    pub(crate) fn total_size(&self) -> usize {
        // size of container is already known
        match self {
            Value::List(list) => return list.as_bytes().len(),
            Value::Map(map) => return map.as_bytes().len(),
            Value::Object(obj) => return obj.as_bytes().len(),
            _ => {}
        }

        let value_type = self.get_type();

        let type_size = value_type.size();

        let fixed_size = value_type.storage.fixed_size();
        let data_size = self.data_size();

        if let Some(data_size) = fixed_size {
            type_size + data_size
        } else if let Some(data_size) = data_size {
            type_size + Size::new(data_size).unwrap().size() + data_size
        } else {
            // value is either fixed size or arbitrary sized
            unreachable!()
        }
    }

    /// Writes this value (\[type\] \[size\] \[data\]) to given buffer
    /// and returns next insert position, how many bytes were written
    pub(crate) fn write<'b>(&self, buf: &'b mut [u8]) -> Result<(&'b mut [u8], usize)> {
        let total_size = self.total_size();

        if buf.len() < total_size {
            return Err(Error::SmallBuffer(total_size - buf.len()));
        }

        match self {
            Value::List(list) => {
                buf[..total_size].copy_from_slice(list.as_bytes());
                return Ok((&mut buf[total_size..], total_size));
            }
            Value::Map(map) => {
                buf[..total_size].copy_from_slice(map.as_bytes());
                return Ok((&mut buf[total_size..], total_size));
            }
            Value::Object(obj) => {
                buf[..total_size].copy_from_slice(obj.as_bytes());
                return Ok((&mut buf[total_size..], total_size));
            }
            _ => {}
        }

        let value_type = self.get_type();

        let fixed_size = value_type.storage.fixed_size();
        let data_size = self.data_size();

        // write [type]
        let mut buf = value_type.write(buf);

        // write [size] if present
        if let Some(mut data_size) = data_size {
            if value_type.storage == Storage::String {
                // for string storage we should not count null terminator in [size] item
                data_size -= 1;
            }
            buf = Size::new(data_size).unwrap().write(buf).unwrap();
        }

        // write [data] if present
        match self {
            Value::Null | Value::True | Value::False | Value::Empty(_) => {}

            Value::UInt8(val) | Value::Byte(_, val) => buf[0] = *val,
            Value::Int8(val) => buf[0] = *val as u8,
            Value::UInt16(val) | Value::Word(_, val) => BigEndian::write_u16(buf, *val),
            Value::Int16(val) => BigEndian::write_i16(buf, *val),
            Value::UInt32(val) | Value::DWord(_, val) => BigEndian::write_u32(buf, *val),
            Value::Int32(val) => BigEndian::write_i32(buf, *val),
            Value::Float(val) => BigEndian::write_f32(buf, *val),
            Value::UInt64(val) | Value::QWord(_, val) => BigEndian::write_u64(buf, *val),
            Value::Int64(val) => BigEndian::write_i64(buf, *val),
            Value::Double(val) => BigEndian::write_f64(buf, *val),

            Value::Text(val)
            | Value::DateTime(val)
            | Value::Date(val)
            | Value::Time(val)
            | Value::DecimalStr(val)
            | Value::UserText(_, val) => buf[..val.len()].copy_from_slice(val.as_bytes()),

            Value::Blob(val) | Value::UserBlob(_, val) => buf[..val.len()].copy_from_slice(val),

            _ => unreachable!(),
        }

        // add [data] size to buffer and return it
        let buf = &mut buf[data_size.or(fixed_size).unwrap()..];

        Ok((buf, total_size))
    }
}

impl<'a> TryFrom<Value<'a>> for List<'a> {
    type Error = Value<'a>;

    fn try_from(value: Value<'a>) -> core::result::Result<Self, Self::Error> {
        if let Value::List(list) = value {
            Ok(list)
        } else {
            Err(value)
        }
    }
}

impl<'a> TryFrom<Value<'a>> for Map<'a> {
    type Error = Value<'a>;

    fn try_from(value: Value<'a>) -> core::result::Result<Self, Self::Error> {
        if let Value::Map(map) = value {
            Ok(map)
        } else {
            Err(value)
        }
    }
}

impl<'a> TryFrom<Value<'a>> for Object<'a> {
    type Error = Value<'a>;

    fn try_from(value: Value<'a>) -> core::result::Result<Self, Self::Error> {
        if let Value::Object(obj) = value {
            Ok(obj)
        } else {
            Err(value)
        }
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Self {
        if value {
            Value::True
        } else {
            Value::False
        }
    }
}

macro_rules! value_from_impl {
    ($value_type:ty, $enum_name:ident) => {
        impl<'a> From<$value_type> for Value<'a> {
            fn from(value: $value_type) -> Self {
                Value::$enum_name(value)
            }
        }
    };
}

value_from_impl!(u8, UInt8);
value_from_impl!(i8, Int8);
value_from_impl!(u16, UInt16);
value_from_impl!(i16, Int16);
value_from_impl!(u32, UInt32);
value_from_impl!(i32, Int32);
value_from_impl!(f32, Float);
value_from_impl!(u64, UInt64);
value_from_impl!(i64, Int64);
value_from_impl!(f64, Double);
value_from_impl!(&'a str, Text);
value_from_impl!(&'a [u8], Blob);
