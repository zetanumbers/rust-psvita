#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nid(pub u32);

impl std::fmt::Debug for Nid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nid(0x{:0>8X})", self.0)
    }
}

#[cfg(feature = "nid-generation")]
impl Nid {
    pub fn generate(name: &[u8]) -> Nid {
        use core::convert::TryInto;
        use sha1::{Digest, Sha1};

        let digest: [u8; 20] = Sha1::digest(name).into();
        let digest_tail: &[u8; 4] = digest[..4].try_into().unwrap();
        Nid(u32::from_le_bytes(*digest_tail))
    }
}

pub mod noname {
    use super::Nid;

    /// Function 	int module_start(SceSize arglen, const void *argp);
    pub const MODULE_START: Nid = Nid(0x935CD196);
    /// Function 	int module_stop(SceSize arglen, const void *argp);
    pub const MODULE_STOP: Nid = Nid(0x79F8E492);
    /// Function 	int module_exit(SceSize arglen, const void *argp);
    pub const MODULE_EXIT: Nid = Nid(0x913482A9);
    /// Function 	int module_bootstart(SceSize arglen, const void *argp);
    pub const MODULE_BOOTSTART: Nid = Nid(0x5C424D40);
    /// Variable 	SceModuleInfo
    pub const MODULE_INFO: Nid = Nid(0x6C2224BA);
    /// Variable 	SceProcessParam
    pub const MODULE_PROC_PARAM: Nid = Nid(0x70FBA1E7);
    /// Variable 	int
    pub const MODULE_SDK_VERSION: Nid = Nid(0x936C8A78);

    pub const NID_TABLE: [(&str, Nid); 7] = [
        ("module_start", MODULE_START),
        ("module_stop", MODULE_STOP),
        ("module_exit", MODULE_EXIT),
        ("module_bootstart", MODULE_BOOTSTART),
        ("module_info", MODULE_INFO),
        ("module_proc_param", MODULE_PROC_PARAM),
        ("module_sdk_version", MODULE_SDK_VERSION),
    ];

    pub fn try_get_predefined(name: &[u8]) -> Option<Nid> {
        for (table_name, nid) in &NID_TABLE {
            if table_name.as_bytes() == name {
                return Some(*nid);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nid_debug_fmt() {
        assert_eq!(format!("{:?}", Nid(0x000A2E54)), "Nid(0x000A2E54)");
    }

    #[cfg(feature = "nid-generation")]
    #[test]
    fn henkaku_nid_example() {
        assert_eq!(Nid(0xEEDA2E54), Nid::generate(b"sceDisplayGetFrameBuf"));
    }
}
