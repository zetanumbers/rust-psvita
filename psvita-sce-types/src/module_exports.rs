use crate::{nid::Nid, Address, USize};
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
/// Entry thread structure - an entry thread is used for executing the
/// module entry functions.
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleEntryThread {
    /// The number of entry thread parameters, typically 3.
    pub num_params: u32,
    /// The initial priority of the entry thread.
    pub init_priority: u32,
    /// The stack size of the entry thread.
    pub stack_size: USize,
    /// The attributes of the entry thread.
    pub attr: u32,
}

/// Size is 0x20 on FW 0.895, 0x30 on FW ??
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceProcessParam {
    /// Size of this struct
    pub size: USize,
    /// "PSP2"
    pub magic: u32,
    /// ex: 1, 5, maybe version
    pub ver: u32,
    /// ex: 0x00895000 for FW 0.895
    pub fw_ver: u32,
    /// ex: "main_thread"
    pub sce_user_main_thread_name: Address,
    /// ex: 0x20, 0xA0, 0x10000100
    pub sce_user_main_thread_priority: i32,
    /// ex: 256 * 1024, 1024 * 1024
    pub sce_user_main_thread_stack_size: u32,
    pub sce_user_main_thread_attribute: u32,
    pub sce_process_name: Address,
    pub sce_process_preload_disabled: u32,
    pub sce_user_main_thread_cpu_affinity_mask: u32,
    /// points to the SceLibcParam
    pub sce_libcparam: Address,
}

/// Size is about 0x28
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceLibcParam {
    /// Size of this structure
    pub size: USize,
    pub unk_4: u32,
    pub sce_libc_heap_size: USize,
    pub sce_libc_heap_size_default: USize,
    pub sce_libc_heap_extended_alloc: u32,
    pub sce_libc_heap_delayed_alloc: u32,
    pub unk_18: u32,
    pub unk_1c: u32,
    pub __sce_libcmallocreplace: Address,
    pub __sce_libcnewreplace: Address,
}

#[cfg(test)]
#[test]
fn type_assertions() {
    use core::mem::size_of;

    assert_eq!(size_of::<SceLibraryEntryCommon>(), 0x10);
    assert_eq!(size_of::<SceLibraryEntrySized1C>(), 0x1C);
    assert_eq!(size_of::<SceLibraryEntrySized20>(), 0x20);
    assert_eq!(size_of::<SceModuleEntryThread>(), 0x10);
    assert_eq!(size_of::<SceProcessParam>(), 0x30);
    assert_eq!(size_of::<SceLibcParam>(), 0x28);
}
