// FIXME: this entire thing should probably be macroified and maybe have compile-time dynamic
// length strings, so that you can specify type ConstString(24) for a 24 char string, or
// something...
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default, Debug, Copy)]
pub struct String32 {
    pub content: [u8; 32],
}

impl String32 {
    pub fn empty() -> Self {
        Self { content: [0x0; 32] }
    }

    pub fn new(str: &str) -> Self {
        let mut a = [0x0; 32];
        a[..str.len().min(32)].copy_from_slice(&str.as_bytes()[..str.len().min(32)]);
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

    pub fn bytes(self) -> [u8; 32] {
        self.content
    }

    pub fn len(self) -> usize {
        let mut len = 0;
        while len < 32 && self.content[len] != 0 {
            len += 1;
        }
        len
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default, Debug, Copy)]
pub struct String8 {
    pub content: [u8; 8],
}

impl String8 {
    pub fn empty() -> Self {
        Self { content: [0x0; 8] }
    }

    pub fn new(str: &str) -> Self {
        let mut a = [0x0; 8];
        a[..str.len().min(8)].copy_from_slice(&str.as_bytes()[..str.len().min(8)]);
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

    pub fn bytes(&mut self) -> [u8; 8] {
        self.content
    }

    pub fn len(self) -> usize {
        let mut len = 0;
        while len < 8 && self.content[len] != 0 {
            len += 1;
        }
        len
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_8() {
        let a = String8::new("");
        let b = String8::new("abc");
        let c = String8::new("abcdefgh");
        let d = String8::new("lmnopqrstuvw");

        assert_eq!(a.str(), "");
        assert_eq!(b.str(), "abc");
        assert_eq!(c.str(), "abcdefgh");
        assert_eq!(d.str(), "lmnopqrs");
    }
}
