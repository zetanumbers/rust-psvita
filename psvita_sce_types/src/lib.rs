use object::{
    endian::{self, LittleEndian},
    pod,
};
use sha1::{Digest, Sha1};

pub type Nid = endian::U32<LittleEndian>;

pub fn generate_nid(name: &[u8]) -> Nid {
    let digest: [u8; 20] = Sha1::digest(name).into();
    let (nid, _tail) = pod::from_bytes(&digest).unwrap();
    *nid
}
