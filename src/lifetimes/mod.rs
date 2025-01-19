pub mod iter_func;
pub mod iter_trait;

pub struct ByteIter<'remainder> {
    pub remainder: &'remainder [u8],
}

impl<'a> ByteIter<'a> {
    pub fn new(remainder: &'a [u8]) -> Self {
        ByteIter { remainder }
    }
}
