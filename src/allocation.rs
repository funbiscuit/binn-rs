/// Represents memory where binn data will be stored
#[non_exhaustive]
#[derive(Debug)]
pub enum Allocation<'a> {
    /// Represents static allocation that can't be changed in size
    /// and will be valid for a lifetime *'a*
    Static(&'a mut [u8]),
}

impl<'a> From<&'a mut [u8]> for Allocation<'a> {
    fn from(value: &'a mut [u8]) -> Self {
        Allocation::Static(value)
    }
}
