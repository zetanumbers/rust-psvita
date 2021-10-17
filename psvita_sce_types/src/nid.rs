use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroable, Pod)]
pub struct Nid(pub u32);

#[cfg(feature = "nid-generation")]
impl Nid {
    pub fn from_bytes(name: &[u8]) -> Nid {
        use core::convert::TryInto;
        use sha1::{Digest, Sha1};

        let digest: [u8; 20] = Sha1::digest(name).into();
        let digest_tail: &[u8; 4] = digest[..4].try_into().unwrap();
        Nid(u32::from_le_bytes(*digest_tail))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "nid-generation")]
    #[test]
    fn henkaku_nid_example() {
        use super::Nid;
        assert_eq!(Nid(0xEEDA2E54), Nid::from_bytes(b"sceDisplayGetFrameBuf"));
    }
}
