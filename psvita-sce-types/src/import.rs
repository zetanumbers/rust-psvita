use crate::{Ptr, USize};
use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use core::fmt;
use std::os::raw::c_char;
use thiserror::Error;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SceModuleImportCommon {
    /// Size of this struct (usually 0x24, 0x2C or 0x34)
    pub size: u16,
    /// Set to 0x1
    pub version: u16,
    /// Set to 0x0
    /// [`crate::SceLibraryAttribute`]
    pub flags: u16,
    /// Number of function imports
    pub num_syms_funcs: u16,
    /// Number of variable imports
    pub num_syms_vars: u16,
    /// Number of TLS variable imports
    pub num_syms_tls_vars: u16,
}

unsafe impl Zeroable for SceModuleExportCommon {}
unsafe impl Pod for SceModuleExportCommon {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct SceModuleImportSized24 {
    pub common: SceModuleImportCommon,
    /// NID of library to import
    pub library_nid: u32,
    /// Pointer to name of imported library, for debugging
    pub library_name: Ptr<c_char>,
    /// Pointer to array of function NIDs to import
    pub func_nid_table: Ptr<u32>,
    /// Pointer to array of stub functions to fill
    pub func_entry_table: Ptr<Ptr<FunctionStubPlaceholder>>,
    /// Pointer to array of variable NIDs to import
    pub var_nid_table: Ptr<u32>,
    /// Pointer to array of data pointers to write to
    pub var_entry_table: Ptr<Ptr<SceVarImportsHeader>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SceModuleImportSized34 {
    pub common: SceModuleImportCommon,
    pub reserved: u32,
    /// NID of library to import
    pub library_nid: u32,
    /// Pointer to name of imported library, for debugging
    pub library_name: Ptr<c_char>,
    /// Usually 0
    pub sce_sdk_version: u32,
    /// Pointer to array of function NIDs to import
    pub func_nid_table: Ptr<u32>,
    /// Pointer to array of stub functions to fill
    pub func_entry_table: Ptr<Ptr<FunctionStubPlaceholder>>,
    /// Pointer to array of variable NIDs to import
    pub var_nid_table: Ptr<u32>,
    /// Pointer to array of data pointers to write to
    pub var_entry_table: Ptr<Ptr<SceVarImportsHeader>>,
    /// Pointer to array of TLS variable NIDs to import
    pub tls_var_nid_table: Ptr<u32>,
    /// Pointer to array of data pointers to write to
    pub tls_var_entry_table: Ptr<Ptr<SceVarImportsHeader>>,
}

unsafe impl Zeroable for SceModuleExportCommon {}
unsafe impl Pod for SceModuleExportCommon {}

pub type SceModuleImport = SceModuleImportSized34;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct FunctionStubPlaceholder {
    pub text: [u32; 4],
}

unsafe impl Zeroable for SceModuleExportCommon {}
unsafe impl Pod for SceModuleExportCommon {}

impl Default for FunctionStubPlaceholder {
    fn default() -> Self {
        Self::FUNCTION_STUB_PLACEHOLDER
    }
}

impl FunctionStubPlaceholder {
    pub fn new() -> Self {
        Self::FUNCTION_STUB_PLACEHOLDER
    }

    pub const FUNCTION_STUB_PLACEHOLDER: Self = Self {
        text: [
            0xe3e00000, // mvn r0, #0
            0xe12fff1e, // bx lr
            0xe1a00000, // mov r0, r0
            0x00000000, // ; padding
        ],
    };
}

bitfield! {
    #[derive(Copy, Clone, Debug)]
    pub struct SceVarImportsHeader(u32);
    impl Debug;
    pub unk, set_unk: 7, 0;
    pub reloc_count, set_reloc_count: 23, 8;
    pub pad, set_pad: 31, 24;
}

unsafe impl Zeroable for SceModuleExportCommon {}
unsafe impl Pod for SceModuleExportCommon {}

#[cfg(test)]
#[test]
fn type_assertions() {
    use core::mem::size_of;

    assert_eq!(size_of::<SceModuleImportCommon>(), 0x0C);
    assert_eq!(size_of::<SceModuleImportSized24>(), 0x24);
    assert_eq!(size_of::<SceModuleImportSized34>(), 0x34);
}
