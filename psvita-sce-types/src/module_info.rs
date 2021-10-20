use crate::{
    module_exports::SceLibraryEntry, module_imports::SceModuleImport, Address, AddressRange,
    PtrRange, USize,
};
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};

pub const MODULE_NAME_MAX_LEN: usize = 27;

bitflags! {
    /// Module type attributes
    #[repr(transparent)]
    #[derive(Zeroable, Pod)]
    pub struct SceModuleAttribute: u16 {
        const CANT_STOP = 0x0001;
        const EXCLUSIVE_LOAD = 0x0002;
        const EXCLUSIVE_START = 0x0004;
    }

    /// Module Privilege Levels - These levels define the permissions a module can have.
    #[repr(transparent)]
    #[derive(Zeroable, Pod)]
    pub struct SceModulePrivilegeLevel: u16 {
        /// Lowest permission
        const USER                 = 0x0000;
        /// MS modeul. POPS/Demo.
        const MS                   = 0x0200;
        /// USB WLAN module. Gamesharin.
        const USBWLAN              = 0x0400;
        /// Application module
        const APP                  = 0x0600;
        /// VSH module
        const VSH                  = 0x0800;
        /// Kernel module. Highest permission.
        const KERNEL               = 0x1000;
        /// The module uses KIRK's memlmd resident library
        const KIRK_MEMLMD_LIB      = 0x2000;
        /// The module uses KIRK's semaphore resident library
        const KIRK_SEMAPHORE_LIB   = 0x4000;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroable, Pod)]
pub struct RawAttributes(pub u16);

impl RawAttributes {
    pub fn new(attrs: SceModuleAttribute, privilege: SceModulePrivilegeLevel) -> Self {
        RawAttributes(attrs.bits() | privilege.bits())
    }

    pub fn try_into_pair(self) -> Option<(SceModuleAttribute, SceModulePrivilegeLevel)> {
        if let 0 =
            self.0 & !(SceModuleAttribute::all().bits() | SceModulePrivilegeLevel::all().bits())
        {
            return None;
        }

        Some((
            SceModuleAttribute::from_bits_truncate(self.0),
            SceModulePrivilegeLevel::from_bits_truncate(self.0),
        ))
    }
}

/// Common beginning of `SceModuleInfo` structs.
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleInfoCommon {
    /// SceModuleAttribute of the module
    pub attributes: RawAttributes,
    /// Major version of the module (usually set to 1) followed by Minor version of the module (usually set to 1)
    pub module_version: [u8; 2],
    /// Name of the module. Null-terminated string.
    pub name: [u8; MODULE_NAME_MAX_LEN],
    /// SceModuleInfo version
    pub info_version: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct PublicApi {
    /// Exports array
    pub exports: PtrRange<SceLibraryEntry>,
    /// Imports array
    pub imports: PtrRange<SceModuleImport>,
}

/// Global pointer value for MIPS, TOC address (address of .toc) for PowerPC, always 0 for ARM
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct GPValue(pub Address);

/// It was wrongly named module NID. It is a sort of hash to ensure integrity and versioning.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct DebugFingerprint(pub u32);

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct Entries {
    /// Offset to module_start function. To disable set it to `-1`
    pub start_entry: Address,
    /// Offset to module_stop function. To disable set it to `-1`
    pub stop_entry: Address,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct TlsInfo {
    /// Offset to start of TLS (Thread Local Storage)
    pub tls_start: Address,
    /// Certainly equals (tls_end - tls_start)
    pub tls_filesz: USize,
    /// Certainly equals (tls_initialized_data_end - tls_start)
    pub tls_memsz: USize,
}

/// Address range of ARM EXIDX (optional)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ArmExidx(pub AddressRange);

/// Address range of ARM EXTAB (optional)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct ArmExtab(pub AddressRange);

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleInfoV0 {
    pub common: SceModuleInfoCommon,
    pub gp_value: GPValue,
    pub public_api: PublicApi,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleInfoV1 {
    pub common: SceModuleInfoCommon,
    pub gp_value: GPValue,
    pub public_api: PublicApi,
    pub debug_fingerprint: DebugFingerprint,
    pub entries: Entries,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleInfoV2 {
    pub common: SceModuleInfoCommon,
    pub gp_value: GPValue,
    pub public_api: PublicApi,
    pub debug_fingerprint: DebugFingerprint,
    pub entries: Entries,
    pub arm_exidx: ArmExidx,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleInfoV3 {
    pub common: SceModuleInfoCommon,
    pub gp_value: GPValue,
    pub public_api: PublicApi,
    pub debug_fingerprint: DebugFingerprint,
    pub entries: Entries,
    pub arm_exidx: ArmExidx,
    pub tls: TlsInfo,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleInfoV6 {
    pub common: SceModuleInfoCommon,
    pub gp_value: GPValue,
    pub public_api: PublicApi,
    pub debug_fingerprint: DebugFingerprint,
    pub tls: TlsInfo,
    pub entries: Entries,
    pub arm_exidx: ArmExidx,
    pub arm_extab: ArmExtab,
}

pub type SceModuleInfo = SceModuleInfoV6;

#[cfg(test)]
#[test]
fn type_assertions() {
    use core::mem::size_of;

    assert_eq!(
        SceModuleAttribute::all().bits() & SceModulePrivilegeLevel::all().bits(),
        0
    );
    assert_eq!(size_of::<SceModuleInfoCommon>(), 0x20);
    assert_eq!(size_of::<SceModuleInfoV0>(), 0x34);
    assert_eq!(size_of::<SceModuleInfoV1>(), 0x40);
    assert_eq!(size_of::<SceModuleInfoV2>(), 0x48);
    assert_eq!(size_of::<SceModuleInfoV3>(), 0x54);
    assert_eq!(size_of::<SceModuleInfoV6>(), 0x5C);
}
