/// Write a function that truncates words in a string to length n.
use std::str::Chars;

// TruncatingIterator takes a string and a number (size), and implements iterator that iterates through each
// character. Words in the string are truncated to the supplied size.
pub struct TruncatingIterator<'a> {
    size: usize,
    idx: usize,
    chars: Chars<'a>,
}

impl<'a> TruncatingIterator<'a> {
    pub fn new(input: &'a str, size: usize) -> Self {
        Self {
            chars: input.chars(),
            size,
            idx: 0,
        }
    }
}

impl<'a> Iterator for TruncatingIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.chars.next()?;

        if !c.is_alphabetic() {
            self.idx = 0;
            return Some(c);
        }

        if self.idx < self.size {
            self.idx += 1;
            return Some(c);
        }

        self.next()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn only_alphanumeric() {
        let iter = TruncatingIterator::new("Never gonna give you up", 3);
        assert_eq!(iter.collect::<String>(), "Nev gon giv you up");
    }

    #[test]
    fn special_chars() {
        let iter = TruncatingIterator::new("*Hello* darkness, my ~old_friend", 3);
        assert_eq!(iter.collect::<String>(), "*Hel* dar, my ~old_fri");
    }
}
