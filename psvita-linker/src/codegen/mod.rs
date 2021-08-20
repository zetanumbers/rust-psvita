mod verification;

type VitaEndian = object::LittleEndian;
const VITA_ENDIAN: VitaEndian = VitaEndian {};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
    Function,
    Variable,
    TLS,
}

