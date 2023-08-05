use crate::error::Result;
use crate::{utils, Error, List, Map, Object, Value};
use byteorder::{BigEndian, ByteOrder};
use core::marker::PhantomData;

use crate::size::Size;
use crate::Allocation;
use core::ptr::NonNull;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key<'a> {
    Empty,
    Num(i32),
    Str(&'a str),
}

impl<'a> Key<'a> {
    /// Returns how many bytes this key will take in serialized form
    pub fn size(&self) -> usize {
        match self {
            Key::Empty => 0,
            Key::Num(_) => 4,
            Key::Str(val) => val.len() + 1,
        }
    }

    pub fn to_num(self) -> Option<i32> {
        match self {
            Key::Num(num) => Some(num),
            _ => None,
        }
    }

    pub fn to_str(self) -> Option<&'a str> {
        match self {
            Key::Str(text) => Some(text),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyType {
    Empty,
    Num,
    Str,
}

/// Base internal type for all containers
///
/// Clone is derived for convenience, should be used with caution
#[derive(Clone, Debug, Eq)]
pub struct RawContainer<'a> {
    buf: NonNull<[u8]>,
    count: Size,
    key_type: KeyType,
    len: Size,
    mutable: bool,
    parent: Option<NonNull<RawContainer<'a>>>,

    _marker: PhantomData<&'a [u8]>,
}

impl<'a> PartialEq for RawContainer<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.as_bytes() == other.as_bytes()
    }
}

impl<'a> RawContainer<'a> {
    /// Adds new container field
    ///
    /// Returns mutable container inside this container, so it can be modified
    pub fn add_container(
        &mut self,
        key: Key<'_>,
        container: &RawContainer<'_>,
    ) -> Result<RawContainer<'_>> {
        self.check_available_size(key.size() + container.len.value())?;

        let key_size = self.ensure_mutable()?.write_key(key)?;

        let parent = NonNull::new(self as *mut RawContainer<'_>);

        // write key doesn't update container size, so insert position
        // will be at the beginning of the key
        let buf = &mut self.insert_position()[key_size..];

        // size is already checked, no error possible
        buf[..container.len.value()].copy_from_slice(container.as_bytes());

        // create new container, that will point inside our buffer
        let inner = RawContainer {
            buf: buf.into(),
            count: container.count,
            key_type: self.key_type,
            len: container.len,
            mutable: true,
            parent,
            _marker: PhantomData,
        };

        self.increment_size_and_count(key_size + container.len.value(), 1);

        Ok(inner)
    }

    /// Adds new field with given name and value
    pub fn add_value<'c, 'p: 'c, 'd>(
        &'p mut self,
        key: Key<'_>,
        value: Value<'d>,
    ) -> Result<Value<'c>> {
        // addition of container is handled separately
        match value {
            Value::List(list) => {
                let inner = self.add_container(key, &list.inner)?;
                return Ok(Value::List(List { inner }));
            }
            Value::Map(map) => {
                let inner = self.add_container(key, &map.inner)?;
                return Ok(Value::Map(Map { inner }));
            }
            Value::Object(obj) => {
                let inner = self.add_container(key, &obj.inner)?;
                return Ok(Value::Object(Object { inner }));
            }
            _ => {}
        }

        let data_size = value.total_size();
        self.check_available_size(key.size() + data_size)?;

        let key_size = self.ensure_mutable()?.write_key(key)?;

        // write key doesn't update container size, so len
        // will be at the beginning of the key
        let len_with_key = self.len.value() + key_size;

        let len_with_key = len_with_key + self.increment_size_and_count(key_size + data_size, 1);

        let buf = &mut self.as_bytes_mut()[len_with_key..];

        // size is already checked, no error possible
        value.write(buf).unwrap();

        // skip size and type entry, used for restoring of text and blob
        let buf = &buf[value.get_type().size()..];
        let buf = if let Some(size) = value.data_size() {
            &buf[Size::new(size).unwrap().size()..]
        } else {
            // buf later is used only for arbitrary sized values (text and blob)
            &[]
        };

        // rebuild value
        let value = match value {
            Value::Null => Value::Null,
            Value::True => Value::True,
            Value::False => Value::False,
            Value::UInt8(v) => Value::UInt8(v),
            Value::Int8(v) => Value::Int8(v),
            Value::UInt16(v) => Value::UInt16(v),
            Value::Int16(v) => Value::Int16(v),
            Value::UInt32(v) => Value::UInt32(v),
            Value::Int32(v) => Value::Int32(v),
            Value::Float(v) => Value::Float(v),
            Value::UInt64(v) => Value::UInt64(v),
            Value::Int64(v) => Value::Int64(v),
            Value::Double(v) => Value::Double(v),
            Value::Text(text) => {
                let text = &buf[..text.len()];
                Value::Text(core::str::from_utf8(text).unwrap())
            }
            Value::DateTime(text) => {
                let text = &buf[..text.len()];
                Value::DateTime(core::str::from_utf8(text).unwrap())
            }
            Value::Date(text) => {
                let text = &buf[..text.len()];
                Value::Date(core::str::from_utf8(text).unwrap())
            }
            Value::Time(text) => {
                let text = &buf[..text.len()];
                Value::Time(core::str::from_utf8(text).unwrap())
            }
            Value::DecimalStr(text) => {
                let text = &buf[..text.len()];
                Value::DecimalStr(core::str::from_utf8(text).unwrap())
            }
            Value::Blob(bytes) => Value::Blob(&buf[..bytes.len()]),
            Value::Empty(sub) => Value::Empty(sub),
            Value::Byte(sub, v) => Value::Byte(sub, v),
            Value::Word(sub, v) => Value::Word(sub, v),
            Value::DWord(sub, v) => Value::DWord(sub, v),
            Value::QWord(sub, v) => Value::QWord(sub, v),
            Value::UserText(sub, text) => {
                let text = &buf[..text.len()];
                Value::UserText(sub, core::str::from_utf8(text).unwrap())
            }
            Value::UserBlob(sub, bytes) => Value::UserBlob(sub, &buf[..bytes.len()]),
            _ => unreachable!(),
        };

        Ok(value)
    }

    /// Returns slice of bytes representing current container
    /// Only actually used bytes are included
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            //SAFETY: buf is valid for lifetime of ContainerState
            &self.buf.as_ref()[..self.len.value()]
        }
    }

    pub fn count(&self) -> usize {
        self.count.value()
    }

    /// Create read-only container from given slice
    pub fn from_bytes(bytes: &[u8], key_type: KeyType) -> Result<RawContainer<'_>> {
        // skip type byte
        let len: Size = bytes[1..].try_into()?;
        let count: Size = bytes[(len.size() + 1)..].try_into()?;

        let container = RawContainer {
            buf: bytes.into(),
            count,
            key_type,
            len,
            mutable: false,
            parent: None,
            _marker: PhantomData,
        };

        // check that all items in container can be parsed when iterated
        if container.iter().count() == count.value() {
            Ok(container)
        } else {
            Err(Error::Malformed)
        }
    }

    /// Create writable container from given allocation
    ///
    /// Allocation must contain valid container data
    pub fn new_mut(allocation: Allocation<'_>, key_type: KeyType) -> Result<RawContainer<'_>> {
        let container = match allocation {
            Allocation::Static(bytes) => Self::from_bytes(bytes, key_type)?,
        };

        // we have mutable pointer for storage
        Ok(RawContainer {
            mutable: true,
            ..container
        })
    }

    pub fn get(&self, key: Key<'_>) -> Option<Value<'_>> {
        self.iter()
            .find(|(item_key, _)| item_key == &key)
            .map(|(_, v)| v)
    }

    pub fn get_at(&self, pos: usize) -> Option<Value<'_>> {
        self.iter().nth(pos).map(|(_, v)| v)
    }

    pub fn iter(&self) -> RawIterator<'_> {
        RawIterator {
            container: self,
            cursor: 1 + self.len.size() + self.count.size(), // skip header of container
            key_type: self.key_type,
        }
    }

    /// Returns slice of bytes representing current container
    ///
    /// # Panics:
    ///
    /// Panics if this container is readonly
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.ensure_mutable().expect("container is readonly");
        unsafe {
            //SAFETY: buf is valid for lifetime of ContainerState
            // if readonly is false, than this buffer was created from mutable slice
            self.buf.as_mut()
        }
    }
    /// Check if new item with given size can be added
    /// Size must include key size
    fn check_available_size(&mut self, item_size: usize) -> Result<()> {
        let len = self.len.value();
        let mut new_len = len + item_size;
        // adjust len if len or count can't be compacted anymore
        if self.len.is_u8() && !Size::is_compactable(new_len) {
            new_len += 3;
        }
        if self.count.is_u8() && !Size::is_compactable(self.count.value() + 1) {
            new_len += 3;
        }

        let buf = self.insert_position();

        if buf.len() < new_len - len {
            Err(Error::SmallBuffer(new_len - len - buf.len()))
        } else {
            Ok(())
        }
    }

    /// Checks that container, otherwise returns error
    fn ensure_mutable(&mut self) -> Result<&mut Self> {
        if self.mutable {
            Ok(self)
        } else {
            Err(Error::ReadOnly)
        }
    }

    /// Updates parent of this container
    ///
    /// Returns size of data shift in case size or count turned from compact to full form
    fn increment_size_and_count(&mut self, extra_size: usize, extra_count: usize) -> usize {
        let new_count = Size::new(self.count.value() + extra_count).unwrap();

        let mut data_start = 3;
        if !self.len.is_u8() {
            data_start += 3
        }
        if !self.count.is_u8() {
            data_start += 3
        }

        let mut shift = 0;
        if self.count.is_u8() && !new_count.is_u8() {
            shift += 3;
        }

        // assume that we never create containers of size > 2GiB
        let new_len = Size::new(self.len.value() + extra_size + shift).unwrap();
        if self.len.is_u8() && !new_len.is_u8() {
            shift += 3;
        }
        let new_len = Size::new(self.len.value() + extra_size + shift).unwrap();
        // shift all data if len or count switched from compact form to full form

        if shift > 0 {
            let data_end = self.as_bytes_mut().len() - shift;
            self.as_bytes_mut()
                .copy_within(data_start..data_end, data_start + shift);
        }

        // create len again since shift might change
        self.len = new_len;
        self.count = new_count;

        // size of buffer is already checked
        let buf = self.as_bytes_mut();
        let buf = new_len.write(&mut buf[1..]).unwrap();
        new_count.write(buf).unwrap();

        if let Some(parent) = self.parent_mut() {
            parent.increment_size_and_count(extra_size + shift, 0);
        }

        shift
    }

    /// Returns place where new items should be added
    fn insert_position(&mut self) -> &mut [u8] {
        let len = self.len.value();
        &mut self.as_bytes_mut()[len..]
    }

    /// Returns parent container of this container
    fn parent_mut(&mut self) -> Option<&mut RawContainer<'a>> {
        if let Some(mut parent) = self.parent {
            // SAFETY: parent must be valid since lifetime of this document
            // is tied to lifetime of parent and it's buffer
            unsafe { Some(parent.as_mut()) }
        } else {
            None
        }
    }

    /// Begins new item (writes its key) and returns how many bytes were written
    ///
    /// Container count and size is not updated
    fn write_key(&mut self, key: Key<'_>) -> Result<usize> {
        match key {
            Key::Empty => Ok(0),
            Key::Num(key) => self.write_i32_key(key),
            Key::Str(key) => self.write_str_key(key),
        }
    }

    /// Writes str key and returns how many bytes were written
    ///
    /// Size and count are not updated
    fn write_str_key(&mut self, key: &str) -> Result<usize> {
        let key = key.as_bytes();

        if key.len() > 255 {
            return Err(Error::LongKey);
        }

        // key does not include null terminator but includes key length (1 byte)
        let key_size = key.len() + 1;

        // size of buffer is already checked
        let buf = self.insert_position();

        // do not include byte with size
        buf[0] = key_size as u8 - 1;
        buf[1..key_size].copy_from_slice(key);

        Ok(key_size)
    }

    /// Writes i32 key and returns how many bytes were written
    ///
    /// Size and count are not updated
    fn write_i32_key(&mut self, key: i32) -> Result<usize> {
        // size of buffer is already checked
        BigEndian::write_i32(self.insert_position(), key);

        Ok(4)
    }
}

pub struct RawIterator<'a> {
    container: &'a RawContainer<'a>,
    cursor: usize,
    key_type: KeyType,
}

impl<'a> Iterator for RawIterator<'a> {
    type Item = (Key<'a>, Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.container.len.value() {
            return None;
        }
        let buf = &self.container.as_bytes()[self.cursor..];
        let key = match self.key_type {
            KeyType::Empty => Key::Empty,
            KeyType::Num => {
                let num = utils::read_i32(buf).ok()?;
                Key::Num(num)
            }
            KeyType::Str => {
                let key = utils::read_key(buf).ok()?;
                Key::Str(key)
            }
        };
        self.cursor += key.size();
        let buf = &self.container.as_bytes()[self.cursor..];
        let value = Value::deserialize(buf).ok()?;
        self.cursor += value.total_size();

        Some((key, value))
    }
}
