/// Subtype of some type.
///
/// Sub types in binn are defined as numbers in range from 0 to 4096 (excluding 4096).
///
/// Used to create values of user-defined types:
/// ```
/// use binn_rs::{SubType, Value};
///
/// const CUSTOM_TEXT: SubType = SubType::new_unchecked(7);
///
/// let value = Value::UserText(CUSTOM_TEXT, "some text that means something");
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubType(pub(crate) u16);

macro_rules! impl_try_from {
    ($name:ident) => {
        impl TryFrom<$name> for SubType {
            type Error = crate::error::OutOfRangeError<$name>;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                if (0..4096).contains(&value) {
                    Ok(Self(value as u16))
                } else {
                    Err(crate::error::OutOfRangeError {
                        min: Some(0),
                        max: Some(4095),
                        value,
                    })
                }
            }
        }
    };
}

impl_try_from!(u16);
impl_try_from!(i16);
impl_try_from!(u32);
impl_try_from!(i32);
impl_try_from!(u64);
impl_try_from!(i64);

impl From<u8> for SubType {
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}

impl SubType {
    /// Creates new sub type from given value
    ///
    /// # Panics
    ///
    /// Panics if value is greater than 4095 so it can't
    /// be a valid subtype
    pub const fn new_unchecked(value: u16) -> Self {
        assert!(value < 4096);
        Self(value)
    }

    /// Creates new sub type from given value
    ///
    /// Returns None if value is greater than 4095 so it can't
    /// be a valid subtype
    pub const fn new(value: u16) -> Option<Self> {
        if value < 4096 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns numeric representation of this sub type
    pub const fn value(&self) -> u16 {
        self.0
    }
}
