/// StrSplit is an implememtation of the STD libs filter annd split functions.
/// It explores the role of lifetimes and the sharp corners that come along with them.
/// The puprose of this lib is entirely to explore lifetimes and is based on Jon Gjengset's video:
/// https://www.youtube.com/watch?v=rAl-9HwD858
///
/// StrSplit searches for the delimeter in a string and returns that piece.
/// It also mutates the string in place thus the remainder struct.
pub struct StrSplit<'a> {
    /// Rest of the string left over
    /// Initially, the whole string
    remainder: &'a str,
    /// Delimeter is the pattern that we're looking for.
    delimeter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        StrSplit {
            remainder: haystack,
            delimeter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimeter) {
            let until_delimeter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim..];
            Some(until_delimeter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(&haystack, " ").into_iter();
    //assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}
