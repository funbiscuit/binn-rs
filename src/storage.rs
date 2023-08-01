#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Storage {
    NoBytes = 0x00,
    Byte = 0x20,
    Word = 0x40,
    DWord = 0x60,
    QWord = 0x80,
    String = 0xA0,
    Blob = 0xC0,
    Container = 0xE0,
}

impl Storage {
    /// Return how many bytes this storage takes if it is fixed size.
    /// Otherwise returns None.
    pub fn fixed_size(&self) -> Option<usize> {
        match self {
            Storage::NoBytes => Some(0),
            Storage::Byte => Some(1),
            Storage::Word => Some(2),
            Storage::DWord => Some(4),
            Storage::QWord => Some(8),
            Storage::String | Storage::Blob | Storage::Container => None,
        }
    }
}

impl TryFrom<u8> for Storage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let storage = match value {
            0x00 => Storage::NoBytes,
            0x20 => Storage::Byte,
            0x40 => Storage::Word,
            0x60 => Storage::DWord,
            0x80 => Storage::QWord,
            0xA0 => Storage::String,
            0xC0 => Storage::Blob,
            0xE0 => Storage::Container,
            _ => return Err(()),
        };
        Ok(storage)
    }
}
