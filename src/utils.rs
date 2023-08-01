use crate::error::Error;
use crate::error::Result;
use crate::size::Size;

macro_rules! read_num_impl {
    ($name:ident) => {
        paste::paste! {
            pub fn [<read_ $name >](buf: &[u8]) -> crate::error::Result<$name> {
                if buf.len() < core::mem::size_of::<$name>() {
                    return Err(crate::error::Error::Malformed);
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

pub fn read_u8(buf: &[u8]) -> Result<u8> {
    if buf.is_empty() {
        return Err(Error::Malformed);
    }

    Ok(buf[0])
}

pub fn read_i8(buf: &[u8]) -> Result<i8> {
    if buf.is_empty() {
        return Err(Error::Malformed);
    }

    Ok(buf[0] as i8)
}

/// Reads single key from buffer and returns it with how many bytes were read
pub fn read_key(buf: &[u8]) -> Result<&str> {
    if buf.is_empty() {
        return Err(Error::Malformed);
    }
    let len = buf[0] as usize;
    let buf = &buf[1..];
    if buf.len() < len {
        return Err(Error::Malformed);
    }

    let key = core::str::from_utf8(&buf[..len]).map_err(|_| Error::Malformed)?;

    Ok(key)
}

pub fn read_text(buf: &[u8]) -> Result<&str> {
    let size = Size::try_from(buf).map_err(|_| Error::Malformed)?;
    let buf = &buf[size.size()..];
    if buf.len() < size.value() {
        return Err(Error::Malformed);
    }

    let text = core::str::from_utf8(&buf[..size.value()]).map_err(|_| Error::Malformed)?;

    Ok(text)
}

pub fn read_blob(buf: &[u8]) -> Result<&[u8]> {
    let size = Size::try_from(buf).map_err(|_| Error::Malformed)?;
    let buf = &buf[size.size()..];
    if buf.len() < size.value() {
        return Err(Error::Malformed);
    }

    Ok(&buf[..size.value()])
}
