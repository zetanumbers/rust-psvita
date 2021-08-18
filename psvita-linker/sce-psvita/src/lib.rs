use sha1::{Digest, Sha1};
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Nid([u8; 4]);

impl From<&'_ str> for Nid {
    fn from(name: &'_ str) -> Self {
        Nid(Sha1::digest(name.as_bytes())[..4].try_into().unwrap())
    }
}

impl From<u32> for Nid {
    fn from(raw: u32) -> Self {
        Nid(raw.to_le_bytes())
    }
}
