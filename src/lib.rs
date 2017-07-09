#![warn(missing_docs)]

//! A [`SplitWhitespace`](https://doc.rust-lang.org/stable/std/str/struct.SplitWhitespace.html)
//! implementation that has a `rest_as_slice` method for getting the rest of the string
//! slice.

/// Iterator over substrings split by whitespace.
pub struct SplitWhitespace<'a> {
    slice: &'a str,
}

impl<'a> SplitWhitespace<'a> {
    /// Creates a new `SplitWhitespace` from `slice`.
    pub fn new(slice: &'a str) -> Self {
        Self { slice }
    }
    /// Returns the rest of the sttring slice.
    ///
    /// ```
    /// # use split_whitespace_rest::SplitWhitespace;
    /// let mut sw = SplitWhitespace::new("say Hello, World!");
    /// assert_eq!(sw.next(), Some("say"));
    /// assert_eq!(sw.rest_as_slice(), "Hello, World!");
    /// ```
    pub fn rest_as_slice(&self) -> &str {
        self.slice
    }
}

impl<'a> Iterator for SplitWhitespace<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.slice.find(|c: char| c.is_whitespace()) {
                Some(offset) => {
                    let sub = &self.slice[..offset];
                    self.slice = &self.slice[offset + 1..];
                    if sub.is_empty() {
                        continue;
                    }
                    break Some(sub);
                }
                None => {
                    if !self.slice.is_empty() {
                        let ret = Some(self.slice);
                        self.slice = &self.slice[self.slice.len()..];
                        break ret;
                    } else {
                        break None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn words() {
        let string = "These are some words";
        let mut swex = SplitWhitespace::new(string);
        assert_eq!(swex.next(), Some("These"));
        assert_eq!(swex.next(), Some("are"));
        assert_eq!(swex.next(), Some("some"));
        assert_eq!(swex.next(), Some("words"));
    }
    #[test]
    fn rest() {
        let string = "say joe Hey Joe, what's up?";
        let mut swex = SplitWhitespace::new(string);
        assert_eq!(swex.next(), Some("say"));
        assert_eq!(swex.next(), Some("joe"));
        assert_eq!(swex.rest_as_slice(), "Hey Joe, what's up?");
    }
    #[test]
    fn multiple_space() {
        let string = "word1   word2  word3     word4 word5    some  more   text ";
        let mut swex = SplitWhitespace::new(string);
        assert_eq!(swex.next(), Some("word1"));
        assert_eq!(swex.next(), Some("word2"));
        assert_eq!(swex.next(), Some("word3"));
        assert_eq!(swex.next(), Some("word4"));
        assert_eq!(swex.next(), Some("word5"));
        assert_eq!(swex.rest_as_slice(), "   some  more   text ");
    }
    #[test]
    fn space_at_end() {
        let string = "hello world       ";
        let mut swex = SplitWhitespace::new(string);
        assert_eq!(swex.next(), Some("hello"));
        assert_eq!(swex.next(), Some("world"));
        assert_eq!(swex.next(), None);
    }
    #[test]
    fn one_word_only_once() {
        let string = "hello";
        let mut swex = SplitWhitespace::new(string);
        assert_eq!(swex.next(), Some("hello"));
        assert_eq!(swex.next(), None);
    }
}
