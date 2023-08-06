use crate::error::DeserializeError;
use crate::size::Size;

macro_rules! read_num_impl {
    ($name:ident) => {
        paste::paste! {
            pub fn [<read_ $name >](buf: &[u8]) -> Result<$name, crate::error::DeserializeError> {
                if buf.len() < core::mem::size_of::<$name>() {
                    return Err(crate::error::DeserializeError::InvalidData);
                }

                Ok(<::byteorder::BigEndian as ::byteorder::ByteOrder>::[<read_ $name >](buf))
            }
        }
    };
}

read_num_impl!(u16);
read_num_impl!(i16);
read_num_impl!(u32);
read_num_impl!(i32);
read_num_impl!(u64);
read_num_impl!(i64);

read_num_impl!(f32);
read_num_impl!(f64);

pub fn read_u8(buf: &[u8]) -> Result<u8, DeserializeError> {
    if buf.is_empty() {
        return Err(DeserializeError::InvalidData);
    }

    Ok(buf[0])
}

pub fn read_i8(buf: &[u8]) -> Result<i8, DeserializeError> {
    if buf.is_empty() {
        return Err(DeserializeError::InvalidData);
    }

    Ok(buf[0] as i8)
}

/// Reads single key from buffer and returns it with how many bytes were read
pub fn read_key(buf: &[u8]) -> Result<&str, DeserializeError> {
    if buf.is_empty() {
        return Err(DeserializeError::InvalidData);
    }
    let len = buf[0] as usize;
    let buf = &buf[1..];
    if buf.len() < len {
        return Err(DeserializeError::InvalidData);
    }

    let key = core::str::from_utf8(&buf[..len]).map_err(|_| DeserializeError::InvalidData)?;

    Ok(key)
}

pub fn read_text(buf: &[u8]) -> Result<&str, DeserializeError> {
    let size = Size::try_from(buf).map_err(|_| DeserializeError::InvalidData)?;
    let buf = &buf[size.size()..];
    if buf.len() < size.value() {
        return Err(DeserializeError::InvalidData);
    }

    let text =
        core::str::from_utf8(&buf[..size.value()]).map_err(|_| DeserializeError::InvalidData)?;

    Ok(text)
}

pub fn read_blob(buf: &[u8]) -> Result<&[u8], DeserializeError> {
    let size = Size::try_from(buf).map_err(|_| DeserializeError::InvalidData)?;
    let buf = &buf[size.size()..];
    if buf.len() < size.value() {
        return Err(DeserializeError::InvalidData);
    }

    Ok(&buf[..size.value()])
}
