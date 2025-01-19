use super::ByteIter;

impl<'remainder> ByteIter<'remainder> {
    /// The lifetime in the result here is critical. This is because there's a &mut self and a general u8.
    /// If no explicit lifetime is included, the elided lifetime assumes that result is of the lifetime of the mutable reference.
    /// This becomes especially problematic when we try to use .next_iter() and the compiler assumes that the reference here is still the mutable reference.
    pub fn next_iter<'mut_self>(&'mut_self mut self) -> Option<&'remainder u8> {
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
    assert_eq!(None, bytes.next_iter().take());
}

#[test]
fn test_iter() {
    let remainder = b"1234";
    let mut bytes = ByteIter::new(remainder);
    let byte_1 = bytes.next_iter();
    let byte_2 = bytes.next_iter();
    assert_eq!(b'1', *byte_1.unwrap());
    assert_eq!(b'2', *byte_2.unwrap());
}
