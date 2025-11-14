#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Copy, bincode::Encode, bincode::Decode)]
pub struct StaticString<const L: usize> {
    pub content: [u8; L],
}

impl<const L: usize> StaticString<L> {
    pub const fn empty() -> Self {
        Self { content: [0x0; L] }
    }

    pub fn new(str: &str) -> Self {
        let mut a = [0x0; L];
        a[..str.len().min(L)].copy_from_slice(&str.as_bytes()[..str.len().min(L)]);
        Self { content: a }
    }

    pub fn set_char(&mut self, idx: usize, char: u8) {
        if idx < self.content.len() {
            self.content[idx] = char;
        }
    }

    pub fn str(&self) -> &str {
        str::from_utf8(&self.content[0..self.len()]).unwrap_or_default()
    }

    pub fn bytes(self) -> [u8; L] {
        self.content
    }

    pub fn len(self) -> usize {
        let mut len = 0;
        while len < L && self.content[len] != 0 {
            len += 1;
        }
        len
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

impl<const L: usize> Default for StaticString<L> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_8() {
        let a = StaticString::<8>::new("");
        let b = StaticString::<8>::new("abc");
        let c = StaticString::<8>::new("abcdefgh");
        let d = StaticString::<8>::new("lmnopqrstuvw");

        assert_eq!(a.str(), "");
        assert_eq!(b.str(), "abc");
        assert_eq!(c.str(), "abcdefgh");
        assert_eq!(d.str(), "lmnopqrs");
    }
}
