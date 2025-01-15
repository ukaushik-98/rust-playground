/// StrSplit is an implememtation of the STD libs filter annd split functions.
/// It explores the role of lifetimes and the sharp corners that come along with them.
/// The puprose of this lib is entirely to explore lifetimes and is based on Jon Gjengset's video:
/// https://www.youtube.com/watch?v=rAl-9HwD858
///
/// StrSplit searches for the delimeter in a string and returns that piece.
/// It also mutates the string in place thus the remainder struct.
#[derive(Debug)]
pub struct StrSplit<'a, 'b> {
    /// Rest of the haystack/string left over
    /// Initially, the whole haystack/string
    remainder: Option<&'a str>,
    /// Delimeter is the pattern that we're looking for.
    delimeter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimeter: &'b str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // begin by checking if we even have a haystack/remainder.
        // if we dont, then there's nothing left and we must return None.
        //
        // why the ref mut? we want to persist the changes we make to self.remainder.
        // simply doing let Some(mut remainder) = self.remainder triggers the move semantic
        // and since this is a &str, Copy would trigger. Essentially, every iteration, we'd just
        // be stuck on Some("a b c d e").
        //
        // ref mut, on the other hand, converts remainder into a &mut &str, allowing us to still effect
        // the self.remainder field on our struct and persist our desired state by derferencing the pointer.
        // this is done on line {58}.
        if let Some(ref mut remainder) = self.remainder {
            // Note: starting from this point we're no longer allowed to create an immutable pointer making debug
            // statements like `println!("remainder: {:?}", self.remainder);` invalid.
            // This is because we can't have both an immutable pointer and a mutable pointer occur with overlapping lifetimes!
            //
            // We can however, start using an immutable pointer starting from line {58} because of contravariance!
            // Since remainder is no longer used after line {58}, we can consider that the end of it's lifetime, effectively shortening
            // it to there. From that point, we're free to allocate and deallocate mutable or immutable references as we like.

            // first, find the index of the where the delimeter matches in our string (aka the haystack/remainder)
            // next, pattern match the index out
            if let Some(next_delim) = remainder.find(self.delimeter) {
                // slice the string until the index.
                // this is what we will wrap in Some() and return
                let until_delimeter = &remainder[..next_delim];
                // finally, mutate our haystack/remainder to everything AFTER the delimeter
                // this is also where the lifetime of remainder ends and we're free to start making new references.
                *remainder = &remainder[(next_delim + self.delimeter.len())..];
                Some(until_delimeter)
            } else {
                // we didn't find the delimeter in our haystack so we consume our Option in self.remainder via take
                // this returns whatever was in the option and converts the enum from Some -> None
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(&haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d e ";
    let letters = StrSplit::new(&haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e", ""].into_iter()));
}
