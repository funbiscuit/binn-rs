use crate::error::Result;
use crate::raw_container::{Key, KeyType, RawContainer};
use crate::value::AsValue;
use crate::Allocation;
use crate::{Error, Value};

const EMPTY_LIST: &[u8] = &[0xE0, 0x03, 0x00];
const EMPTY_MAP: &[u8] = &[0xE1, 0x03, 0x00];
const EMPTY_OBJ: &[u8] = &[0xE2, 0x03, 0x00];

/// Container that stores its elements sequentially and provides
/// *get by position* access
#[derive(Debug, Eq, PartialEq)]
pub struct List<'a> {
    pub(crate) inner: RawContainer<'a>,
}

/// Container that stores its elements with numeric keys and provides
/// *get by key* access. Keys are of `i32` type
#[derive(Debug, Eq, PartialEq)]
pub struct Map<'a> {
    pub(crate) inner: RawContainer<'a>,
}

/// Container that stores its elements with numeric keys and provides
/// *get by key* access. Keys are of `&str` type
#[derive(Debug, Eq, PartialEq)]
pub struct Object<'a> {
    pub(crate) inner: RawContainer<'a>,
}

impl<'a> List<'a> {
    /// Adds new value to this list
    pub fn add_value<'c, 'p: 'c, 'd>(&'p mut self, value: impl AsValue<'d>) -> Result<Value<'c>> {
        self.inner.add_value(Key::Empty, value)
    }

    /// Returns slice of bytes representing current document.
    ///
    /// It's guaranteed to be subslice of initial buffer
    /// and for root document it will starting at 0.
    ///
    /// So by taking it's len you can get how many bytes in buffer are used
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Returns number of elements in this list
    pub fn count(&self) -> usize {
        self.inner.count()
    }

    /// Returns new empty list
    ///
    /// List is read only so no new elements can be added to it
    pub fn empty() -> List<'static> {
        // EMPTY_LIST can't be malformed
        List {
            inner: RawContainer::from_bytes(EMPTY_LIST, KeyType::Empty).unwrap(),
        }
    }

    /// Creates a new list that uses given allocation for storage
    pub fn empty_mut(allocation: impl Into<Allocation<'a>>) -> Result<Self> {
        Ok(Self {
            inner: empty_mut(allocation.into(), EMPTY_LIST, KeyType::Empty)?,
        })
    }

    /// Get value at position
    pub fn get(&self, pos: usize) -> Option<Value<'_>> {
        self.inner.get_at(pos)
    }

    /// Iterate over elements of this list
    pub fn iter(&self) -> impl Iterator<Item = Value<'_>> {
        self.inner.iter().map(|(_, v)| v)
    }
}

impl<'a> Map<'a> {
    /// Adds new field with given name and value to this object
    pub fn add_value<'c, 'p: 'c, 'd>(
        &'p mut self,
        key: i32,
        value: impl AsValue<'d>,
    ) -> Result<Value<'c>> {
        self.inner.add_value(Key::Num(key), value)
    }

    /// Returns slice of bytes representing current document.
    ///
    /// It's guaranteed to be subslice of initial buffer
    /// and for root document it will starting at 0.
    ///
    /// So by taking it's len you can get how many bytes in buffer are used
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Returns number of elements in this map
    pub fn count(&self) -> usize {
        self.inner.count()
    }

    /// Returns new empty object
    ///
    /// Object is read only so no new elements can be added to it
    pub fn empty() -> Map<'static> {
        // EMPTY_MAP can't be malformed
        Map {
            inner: RawContainer::from_bytes(EMPTY_MAP, KeyType::Num).unwrap(),
        }
    }

    /// Creates a new object that uses given allocation for storage
    pub fn empty_mut(allocation: impl Into<Allocation<'a>>) -> Result<Self> {
        Ok(Self {
            inner: empty_mut(allocation.into(), EMPTY_MAP, KeyType::Num)?,
        })
    }

    /// Get value with specific key
    pub fn get(&self, key: i32) -> Option<Value<'_>> {
        self.inner.get(Key::Num(key))
    }

    /// Iterate over elements of this map
    pub fn iter(&self) -> impl Iterator<Item = (i32, Value<'_>)> {
        self.inner.iter().map(|(k, v)| (k.to_num().unwrap(), v))
    }
}

impl<'a> Object<'a> {
    /// Adds new field with given name and value to this object
    pub fn add_value<'c, 'p: 'c, 'd>(
        &'p mut self,
        key: &str,
        value: impl AsValue<'d>,
    ) -> Result<Value<'c>> {
        self.inner.add_value(Key::Str(key), value)
    }

    /// Returns slice of bytes representing current document.
    ///
    /// It's guaranteed to be subslice of initial buffer
    /// and for root document it will starting at 0.
    ///
    /// So by taking it's len you can get how many bytes in buffer are used
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Returns number of elements in this object
    pub fn count(&self) -> usize {
        self.inner.count()
    }

    /// Returns new empty object
    ///
    /// Object is read only so no new elements can be added to it
    pub fn empty() -> Object<'static> {
        // EMPTY_OBJ can't be malformed
        Object {
            inner: RawContainer::from_bytes(EMPTY_OBJ, KeyType::Str).unwrap(),
        }
    }

    /// Creates a new object that uses given allocation for storage
    pub fn empty_mut(allocation: impl Into<Allocation<'a>>) -> Result<Self> {
        Ok(Self {
            inner: empty_mut(allocation.into(), EMPTY_OBJ, KeyType::Str)?,
        })
    }

    /// Get value with specific key
    pub fn get(&self, key: &str) -> Option<Value<'_>> {
        self.inner.get(Key::Str(key))
    }

    /// Iterate over elements of this object
    pub fn iter(&self) -> impl Iterator<Item = (&str, Value<'_>)> {
        self.inner.iter().map(|(k, v)| (k.to_str().unwrap(), v))
    }
}

/// Helper function to create mutable container from given source
fn empty_mut<'a>(
    mut allocation: Allocation<'a>,
    source: &[u8],
    key_type: KeyType,
) -> Result<RawContainer<'a>> {
    let len = source.len();
    match &mut allocation {
        Allocation::Static(buf) => {
            if buf.len() < len {
                return Err(Error::SmallBuffer(len - buf.len()));
            }
            buf[..len].copy_from_slice(source);
        }
    }

    Ok(RawContainer::new_mut(allocation, key_type).unwrap())
}

impl<'a> AsValue<'a> for List<'a> {
    fn to_value(self) -> Value<'a> {
        let inner = self.inner.clone();
        Value::List(List { inner })
    }
}

impl<'a> AsValue<'a> for Map<'a> {
    fn to_value(self) -> Value<'a> {
        let inner = self.inner.clone();
        Value::Map(Map { inner })
    }
}

impl<'a> AsValue<'a> for Object<'a> {
    fn to_value(self) -> Value<'a> {
        let inner = self.inner.clone();
        Value::Object(Object { inner })
    }
}
