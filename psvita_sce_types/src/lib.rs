use object::endian::LittleEndian;
use sha1::{Digest, Sha1};

pub type Nid = object::endian::U32<LittleEndian>;

pub fn generate_nid(name: &[u8]) -> Nid {
    let digest: [u8; 20] = Sha1::digest(name).into();
    let (nid, _tail) = object::pod::from_bytes(&digest[..4]).unwrap();
    *nid
}
