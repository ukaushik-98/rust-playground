struct ByteIter<'remainder> {
    remainder: &'remainder [u8],
}

impl<'a> ByteIter<'a> {
    fn new(remainder: &'a [u8]) -> Self {
        ByteIter { remainder }
    }

    fn next<'mut_self>(&'mut_self mut self) -> Option<&'a u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let result = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(result)
        }
    }
}

#[test]
fn test_empty() {
    let remainder = b"";
    let mut bytes = ByteIter::new(remainder);
    assert_eq!(None, bytes.next().take());
}

#[test]
fn test_iter() {
    let remainder = b"1234";
    let mut bytes = ByteIter::new(remainder);
    let byte_1 = bytes.next();
    let byte_2 = bytes.next();
    assert_eq!(b'1', *byte_1.unwrap());
    assert_eq!(b'2', *byte_2.unwrap());
}
