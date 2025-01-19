use super::ByteIter;

impl<'a> Iterator for ByteIter<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder.is_empty() {
            None
        } else {
            let result: &'a u8 = &self.remainder[0];
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
