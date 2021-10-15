use object::{endian, pod};
use sha1::{Digest, Sha1};

pub type Endian = endian::LittleEndian;
pub type U16 = endian::U16Bytes<Endian>;
pub type U32 = endian::U32Bytes<Endian>;
pub type USize = U32;
pub type Nid = U32;

pub fn generate_nid(name: &[u8]) -> Nid {
    let digest: [u8; 20] = Sha1::digest(name).into();
    let (nid, _tail) = pod::from_bytes(&digest).unwrap();
    *nid
}

pub mod sce_module_info {
    use super::*;

    pub const MODULE_NAME_MAX_LEN: usize = 27;

    /// Common beginning of `SceModuleInfo` structs.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct Common {
        /// Attributes of the module
        pub attributes: U16,
        /// Major version of the module (usually set to 1) followed by Minor version of the module (usually set to 1)
        pub module_version: [u8; 2],
        /// Name of the module. Null-terminated string.
        pub name: [u8; MODULE_NAME_MAX_LEN],
        /// SceModuleInfo version
        pub info_version: u8,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct V6 {
        pub common: Common,
        /// Global pointer value for MIPS, TOC address (address of .toc) for PowerPC, always 0 for ARM
        pub gp_value: USize,
        /// Offset to top of exports array
        pub export_top: USize,
        /// Offset to bottom of exports array
        pub export_bottom: USize,
        /// Offset to top of imports array
        pub import_top: USize,
        /// Offset to bottom of imports array
        pub import_bottom: USize,
        /// It was wrongly named module NID. It is a sort of hash to ensure integrity and versioning.
        pub debug_fingerprint: U32,
        /// Offset to start of TLS (Thread Local Storage)
        pub tls_start: USize,
        /// Certainly equals (tls_end - tls_start)
        pub tls_filesz: USize,
        /// Certainly equals (tls_initialized_data_end - tls_start)
        pub tls_memsz: USize,
        /// Offset to module_start function. To disable set it to:
        /// - `-1` according to [Henkaku wiki](https://wiki.henkaku.xyz/vita/Modules#SceModuleInfo)
        /// - `0` according to [vitasdk toolchain](https://github.com/vitasdk/vita-toolchain/blob/a075d3ab2963d6b12e1a51b6816022d4f0d2c41d/src/sce-elf-defs.h#L36)
        /// TODO: test both
        pub start_entry: USize,
        /// Offset to module_stop function. To disable set it to:
        /// - `-1` according to [Henkaku wiki](https://wiki.henkaku.xyz/vita/Modules#SceModuleInfo)
        /// - `0` according to [vitasdk toolchain](https://github.com/vitasdk/vita-toolchain/blob/a075d3ab2963d6b12e1a51b6816022d4f0d2c41d/src/sce-elf-defs.h#L37)
        /// TODO: test both
        pub stop_entry: USize,
        /// Offset to top of ARM EXIDX (optional)
        pub arm_exidx_top: USize,
        /// Offset to bottom of ARM EXIDX (optional)
        pub arm_exidx_bottom: USize,
        /// Offset to top of ARM EXTAB (optional)
        pub arm_extab_top: USize,
        /// Offset to bottom of ARM EXTAB (optional)
        pub arm_extab_bottom: USize,
    }

    #[cfg(test)]
    #[test]
    fn type_assertions() {
        use core::mem::size_of;
        assert_eq!(size_of::<Common>(), 0x20);
        assert_eq!(size_of::<V6>(), 0x5C);
    }

    unsafe impl pod::Pod for Common {}
    unsafe impl pod::Pod for V6 {}
}
