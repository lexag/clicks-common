use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone, Default, Debug, Copy)]
pub struct String32 {
    content: [u8; 32],
}
