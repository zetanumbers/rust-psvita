use object::elf;
use std::fmt;

use super::{VitaEndian, VITA_ENDIAN};

pub fn validate_header(header: &elf::FileHeader32<VitaEndian>) -> Result<(), VerifyHeaderError> {
    match VerifyHeader::from(header) {
        VerifyHeader::EXPECTED => Ok(()),
        got => Err(VerifyHeaderError(got)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifyHeader {
    pub os_abi: u8,
    pub machine: u16,
    pub e_version: u32,
}

impl From<&'_ elf::FileHeader32<VitaEndian>> for VerifyHeader {
    fn from(header: &'_ elf::FileHeader32<VitaEndian>) -> Self {
        VerifyHeader {
            os_abi: header.e_ident.os_abi,
            machine: header.e_machine.get(VITA_ENDIAN),
            e_version: header.e_version.get(VITA_ENDIAN),
        }
    }
}

impl VerifyHeader {
    pub const EXPECTED: VerifyHeader = VerifyHeader {
        os_abi: elf::ELFOSABI_SYSV,
        machine: elf::EM_ARM,
        e_version: 1,
    };
}

#[derive(Debug)]
pub struct VerifyHeaderError(pub VerifyHeader);

impl std::error::Error for VerifyHeaderError {}

impl fmt::Display for VerifyHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bad psvita elf header (got: {:#?}, expected: {:#?})",
            self.0,
            VerifyHeader::EXPECTED
        )
    }
}
