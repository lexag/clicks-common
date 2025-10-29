use serde::{Deserialize, Serialize};
// FIXME: this entire thing should probably be macroified and maybe have compile-time dynamic
// length strings, so that you can specify type ConstString(24) for a 24 char string, or
// something...
#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone, Default, Debug, Copy)]
pub struct String32 {
    content: [u8; 32],
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

    pub fn str(&mut self) -> &str {
        str::from_utf8(&self.content).unwrap_or_default()
    }

    pub fn bytes(&mut self) -> [u8; 32] {
        self.content
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone, Default, Debug, Copy)]
pub struct String8 {
    content: [u8; 8],
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

    pub fn str(&mut self) -> &str {
        str::from_utf8(&self.content).unwrap_or_default()
    }

    pub fn bytes(&mut self) -> [u8; 8] {
        self.content
    }
}
