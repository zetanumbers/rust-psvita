use crate::{nid::Nid, Address};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceLibraryEntryCommon {
    /// Size of the structure in bytes (usually 0x1C or 0x20)
    pub size: u8,
    /// Unknown. Was added recently.
    pub auxattribute: u8,
    /// Library version (usually 1)
    pub version: u16,
    /// Library attribute flags
    pub attribute: u16,
    /// Number of exported functions
    pub nfunc: u16,
    /// Number of exported variables
    pub nvar: u16,
    /// Number of exported TLS variables
    pub ntls: u16,
    pub hashinfo: HashInfo,
    pub reserved: u8,
    /// Unknown. usually 0
    pub nidaltsets: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceLibraryEntrySized1C {
    pub common: SceLibraryEntryCommon,
    /// Pointer to library name. Set to 0 for NONAME.
    pub libname: Address,
    /// Pointer to array of NIDs of exports
    pub nid_table: Address,
    /// Pointer to array of pointers of exports
    pub entry_table: Address,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceLibraryEntrySized20 {
    pub common: SceLibraryEntryCommon,
    /// Library NID
    pub libname_nid: Nid,
    /// Pointer to library name. Set to 0 for NONAME.
    pub libname: Address,
    /// Pointer to array of NIDs of exports
    pub nid_table: Address,
    /// Pointer to array of pointers of exports
    pub entry_table: Address,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct HashInfo(pub u16);

impl HashInfo {
    pub const LIMIT: u16 = 0x10;

    fn calculate(exports: u16) -> u16 {
        match exports {
            // means exports < HashInfo::LIMIT
            0x00..=0x0F => 0x0,
            0x10..=0x3F => 0x2,
            0x40..=0xFF => 0x4,
            _ => 0x6,
        }
    }

    pub fn new(functions: u16, variables: u16, tls: u16) -> HashInfo {
        HashInfo(
            Self::calculate(functions)
                | Self::calculate(variables) << 4
                | Self::calculate(tls) << 8,
        )
    }
}

#[cfg(test)]
#[test]
fn type_assertions() {
    use core::mem::size_of;

    assert_eq!(size_of::<SceLibraryEntryCommon>(), 0x10);
    assert_eq!(size_of::<SceLibraryEntrySized1C>(), 0x1C);
    assert_eq!(size_of::<SceLibraryEntrySized20>(), 0x20);
}
