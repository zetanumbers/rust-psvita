use crate::{nid::Nid, Address, Ptr, USize};
use bytemuck::{Pod, Zeroable};
use std::os::raw::c_char;

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
    /// Ptr to library name. Set to 0 for NONAME.
    pub libname: Ptr<c_char>,
    /// Ptr to array of NIDs of exports
    pub nid_table: Ptr<Nid>,
    /// Ptr to array of pointers of exports
    pub entry_table: Ptr<Address>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceLibraryEntrySized20 {
    pub common: SceLibraryEntryCommon,
    /// Library NID
    pub libname_nid: Nid,
    /// Ptr to library name. Set to 0 for NONAME.
    pub libname: Ptr<c_char>,
    /// Ptr to array of NIDs of exports
    pub nid_table: Ptr<Nid>,
    /// Ptr to array of pointers of exports
    pub entry_table: Ptr<Address>,
}

pub type SceLibraryEntry = SceLibraryEntrySized20;

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

/// Henkaku wiki says:
/// > Size is 0x20 on FW 0.895, 0x30 on FW ??
///
/// But this struct definition relies upon [vitasdk toolchain's code](https://github.com/vitasdk/vita-toolchain/blob/a075d3ab2963d6b12e1a51b6816022d4f0d2c41d/src/sce-elf-defs.h#L97-L111)
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceProcessParam {
    /// Size of this struct `0x34`
    pub size: USize,
    /// "PSP2"
    pub magic: u32,
    /// ex: could be 6 but also 1 and 5
    pub ver: u32,
    /// SDK version
    /// ex: 0x00895000 for FW 0.895
    pub fw_ver: u32,
    /// ex: "main_thread"
    pub user_main_thread_name: Ptr<c_char>,
    /// ex: 0x20, 0xA0, 0x10000100
    pub user_main_thread_priority: i32,
    /// ex: 256 * 1024, 1024 * 1024
    pub user_main_thread_stack_size: u32,
    /// Unknown
    pub user_main_thread_attribute: u32,
    /// Process name pointer
    pub process_name: Ptr<c_char>,
    /// Module load inhibit
    pub process_preload_disabled: u32,
    /// Unknown
    pub user_main_thread_cpu_affinity_mask: u32,
    pub sce_libc_param: Ptr<SceLibcParam>,
    pub unknown: u32,
}

/// vitasdk toolchain's code [has additional field with description](https://github.com/vitasdk/vita-toolchain/blob/a075d3ab2963d6b12e1a51b6816022d4f0d2c41d/src/sce-elf-defs.h#L167):
/// > default SceLibc heap size - 0x40000 (256KiB)
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceLibcParam {
    /// 0x38
    pub size: u32,
    /// Unknown
    pub unk_04: u32,
    /// Heap size variable
    pub heap_size: Ptr<u32>,
    /// Default heap size variable
    pub default_heap_size: Ptr<u32>,
    /// Dynamically extend heap size
    pub heap_extended_alloc: Ptr<u32>,
    /// Allocate heap on first call to malloc
    pub heap_delayed_alloc: Ptr<u32>,
    /// SDK version
    pub fw_version: u32,
    /// Unknown, set to 9
    pub unk_1c: u32,
    /// malloc replacement functions
    pub malloc_replace: Ptr<MallocReplace>,
    /// new replacement functions
    pub operator_new_replace: Ptr<OperatorNewReplace>,
    /// Dynamically allocated heap initial size
    pub heap_initial_size: Ptr<u32>,
    /// Change alloc unit size from 64k to 1M
    pub heap_unit_1mb: Ptr<u32>,
    /// Detect heap buffer overruns
    pub heap_detect_overrun: Ptr<u32>,
    /// malloc_for_tls replacement functions
    pub malloc_for_tls_replace: Ptr<MallocForTlsReplace>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MallocReplace {
    /// 0x34
    pub size: u32,
    /// Unknown, set to 1
    pub unk_0x4: u32,
    /// Initialize malloc heap
    pub malloc_init: Address,
    /// Terminate malloc heap
    pub malloc_term: Address,
    /// malloc replacement
    pub malloc: Address,
    /// free replacement
    pub free: Address,
    /// calloc replacement
    pub calloc: Address,
    /// realloc replacement
    pub realloc: Address,
    /// memalign replacement
    pub memalign: Address,
    /// reallocalign replacement
    pub reallocalign: Address,
    /// malloc_stats replacement
    pub malloc_stats: Address,
    /// malloc_stats_fast replacement
    pub malloc_stats_fast: Address,
    /// malloc_usable_size replacement
    pub malloc_usable_size: Address,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct OperatorNewReplace {
    /// 0x28
    pub size: u32,
    /// Unknown, set to 1
    pub unk_0x4: u32,
    /// new operator replacement
    pub operator_new: Address,
    /// new (nothrow) operator replacement
    pub operator_new_nothrow: Address,
    /// new[] operator replacement
    pub operator_new_arr: Address,
    /// new[] (nothrow) operator replacement
    pub operator_new_arr_nothrow: Address,
    /// delete operator replacement
    pub operator_delete: Address,
    /// delete (nothrow) operator replacement
    pub operator_delete_nothrow: Address,
    /// delete[] operator replacement
    pub operator_delete_arr: Address,
    /// delete[] (nothrow) operator replacement
    pub operator_delete_arr_nothrow: Address,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MallocForTlsReplace {
    /// 0x18
    pub size: u32,
    /// Unknown, set to 1
    pub unk_0x4: u32,
    /// Initialise tls malloc heap
    pub malloc_init_for_tls: Address,
    /// Terminate tls malloc heap
    pub malloc_term_for_tls: Address,
    /// malloc_for_tls replacement
    pub malloc_for_tls: Address,
    /// free_for_tls replacement
    pub free_for_tls: Address,
}

#[cfg(test)]
#[test]
fn type_assertions() {
    use core::mem::size_of;

    assert_eq!(size_of::<SceLibraryEntryCommon>(), 0x10);
    assert_eq!(size_of::<SceLibraryEntrySized1C>(), 0x1C);
    assert_eq!(size_of::<SceLibraryEntrySized20>(), 0x20);
    assert_eq!(size_of::<SceModuleEntryThread>(), 0x10);
    assert_eq!(size_of::<SceProcessParam>(), 0x34);
    assert_eq!(size_of::<SceLibcParam>(), 0x38);
    assert_eq!(size_of::<MallocReplace>(), 0x34);
    assert_eq!(size_of::<OperatorNewReplace>(), 0x28);
    assert_eq!(size_of::<MallocForTlsReplace>(), 0x18);
}
