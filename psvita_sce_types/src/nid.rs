use bytemuck::{Pod, Zeroable};
use core::convert::TryInto;
use sha1::{Digest, Sha1};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroable, Pod)]
pub struct Nid(pub u32);

impl From<&[u8]> for Nid {
    fn from(name: &[u8]) -> Nid {
        let digest: [u8; 20] = Sha1::digest(name).into();
        let digest_tail: &[u8; 4] = digest[..4].try_into().unwrap();
        Nid(u32::from_le_bytes(*digest_tail))
    }
}

impl Nid {
    pub fn from_bytes(name: &[u8]) -> Nid {
        Nid::from(name)
    }
}
