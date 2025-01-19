struct ByteIter<'a> {
    remainder: &'a [u8],
}

impl<'a> ByteIter<'a> {
    fn next<'mut_self>(&'mut_self mut self) -> Option<&'a u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte: &'a u8 = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}

#[test]
fn test_next() {
    let mut bytes = ByteIter { remainder: b"1" };
    assert_eq!(Some(&b'1'), bytes.next());
    assert_eq!(None, bytes.next());
}

#[test]
fn test_next_iter() {
    let mut bytes = ByteIter { remainder: b"1123" };
    let byte_1 = bytes.next();
    let byte_2 = bytes.next();
    assert_eq!(byte_1, byte_2);
}
