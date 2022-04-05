use crate::{
    helpers::{safe_pod_layout, PodPlaceholder, WriteArtifact},
    library::LibraryAttribute,
    nid::Nid,
    Symbol,
};
use std::{alloc::Layout, ffi::CString};

const PSP2_SDK_VERSION: u32 = 0x03570011;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ExportLibrary {
    /// usually 1
    pub library_version: u16,
    pub attribute: LibraryAttribute,
    pub library_nid: Option<Nid>,
    pub library_name: Option<CString>,
    pub function_table: Vec<(Option<Nid>, Symbol)>,
    pub variable_table: Vec<(Option<Nid>, Symbol)>,
    pub tls_variable_table: Vec<(Option<Nid>, Symbol)>,
}

/// Entry thread structure - an entry thread is used for executing the
/// module entry functions.
#[derive(Copy, Clone, Debug)]
pub struct ModuleEntryThread {
    /// The number of entry thread parameters, typically 3.
    pub num_params: u32,
    /// The initial priority of the entry thread.
    pub init_priority: u32,
    /// The stack size of the entry thread.
    pub stack_size: u32,
    /// The attributes of the entry thread.
    pub attr: u32,
}

#[derive(Clone, Debug)]
pub struct ProcessParam {
    /// ex: "main_thread"
    pub user_main_thread_name: Option<CString>,
    /// ex: 0x20, 0xA0, 0x10000100
    pub user_main_thread_priority: i32,
    /// ex: 256 * 1024, 1024 * 1024
    pub user_main_thread_stack_size: u32,
    pub process_name: Option<CString>,
    /// Module load inhibit
    pub process_preload_disabled: u32,
    pub libc_param: LibcParam,
}

#[derive(Clone, Debug)]
pub struct LibcParam {
    /// Heap size variable
    pub heap_size: Option<Symbol>,
    /// Default heap size variable
    pub default_heap_size: u32,
    /// Dynamically extend heap size
    pub heap_extended_alloc: Option<Symbol>,
    /// Allocate heap on first call to malloc
    pub heap_delayed_alloc: Option<Symbol>,
    /// malloc replacement functions
    pub malloc_replace: MallocReplace,
    /// new replacement functions
    pub operator_new_replace: OperatorNewReplace,
    /// Dynamically allocated heap initial size
    pub heap_initial_size: Option<Symbol>,
    /// Change alloc unit size from 64k to 1M
    pub heap_unit_1mb: Option<Symbol>,
    /// Detect heap buffer overruns
    pub heap_detect_overrun: Option<Symbol>,
    /// malloc_for_tls replacement functions
    pub malloc_for_tls_replace: MallocForTlsReplace,
}

#[derive(Clone, Debug, Default)]
pub struct MallocReplace {
    /// Initialize malloc heap
    pub malloc_init: Option<Symbol>,
    /// Terminate malloc heap
    pub malloc_term: Option<Symbol>,
    /// malloc replacement
    pub malloc: Option<Symbol>,
    /// free replacement
    pub free: Option<Symbol>,
    /// calloc replacement
    pub calloc: Option<Symbol>,
    /// realloc replacement
    pub realloc: Option<Symbol>,
    /// memalign replacement
    pub memalign: Option<Symbol>,
    /// reallocalign replacement
    pub reallocalign: Option<Symbol>,
    /// malloc_stats replacement
    pub malloc_stats: Option<Symbol>,
    /// malloc_stats_fast replacement
    pub malloc_stats_fast: Option<Symbol>,
    /// malloc_usable_size replacement
    pub malloc_usable_size: Option<Symbol>,
}

#[derive(Clone, Debug, Default)]
pub struct OperatorNewReplace {
    /// new operator replacement
    pub operator_new: Option<Symbol>,
    /// new (nothrow) operator replacement
    pub operator_new_nothrow: Option<Symbol>,
    /// new[] operator replacement
    pub operator_new_arr: Option<Symbol>,
    /// new[] (nothrow) operator replacement
    pub operator_new_arr_nothrow: Option<Symbol>,
    /// delete operator replacement
    pub operator_delete: Option<Symbol>,
    /// delete (nothrow) operator replacement
    pub operator_delete_nothrow: Option<Symbol>,
    /// delete[] operator replacement
    pub operator_delete_arr: Option<Symbol>,
    /// delete[] (nothrow) operator replacement
    pub operator_delete_arr_nothrow: Option<Symbol>,
}

#[derive(Clone, Debug, Default)]
pub struct MallocForTlsReplace {
    /// Initialise tls malloc heap
    pub malloc_init_for_tls: Option<Symbol>,
    /// Terminate tls malloc heap
    pub malloc_term_for_tls: Option<Symbol>,
    /// malloc_for_tls replacement
    pub malloc_for_tls: Option<Symbol>,
    /// free_for_tls replacement
    pub free_for_tls: Option<Symbol>,
}

fn hash_info(functions: u16, variables: u16, tls: u16) -> u16 {
    return component(functions) | component(variables) << 4 | component(tls) << 8;

    fn component(exports: u16) -> u16 {
        match exports {
            0x00..=0x0F => 0x0,
            0x10..=0x3F => 0x2,
            0x40..=0xFF => 0x4,
            _ => 0x6,
        }
    }
}

impl WriteArtifact for ProcessParam {
    const SYMBOL: &'static str = "__psvitalinker_ProcessParam";
    const LAYOUT: Layout = safe_pod_layout(13, PodPlaceholder::U32);

    fn write_artifact(&self, artifact: &mut faerie::Artifact) -> Result<(), faerie::ArtifactError> {
        let mut w = Self::write_pod_helper(artifact)?;
        w.u32(Self::LAYOUT.size() as u32);
        w.u8_slice(b"PSP2");
        w.u32(6); // version
        w.u32(PSP2_SDK_VERSION); // SDK version
        w.link_with_cstr(
            0,
            "__psvitalinker_ProcessParam_UserMainThreadName",
            &self.user_main_thread_name,
        )?;

        w.finish()
    }
}

impl WriteArtifact for LibcParam {
    const SYMBOL: &'static str = "__psvitalinker_LibcParam";
    const LAYOUT: Layout = safe_pod_layout(14, PodPlaceholder::U32);

    fn write_artifact(&self, artifact: &mut faerie::Artifact) -> Result<(), faerie::ArtifactError> {
        let mut w = Self::write_pod_helper(artifact)?;
        w.u32(Self::LAYOUT.size() as u32);
        w.u32(0); // unknown
        w.link(0, &self.heap_size)?;
        w.link_with_section(0, Some(&DefaultHeapSize(self.default_heap_size)))?;
        w.link(0, &self.heap_extended_alloc)?;
        w.link(0, &self.heap_delayed_alloc)?;
        w.u32(PSP2_SDK_VERSION); // SDK version
        w.u32(9); // unknown
        w.link_with_section(0, Some(&self.malloc_replace))?;
        w.link(0, &self.heap_initial_size)?;
        w.link(0, &self.heap_unit_1mb)?;
        w.link(0, &self.heap_detect_overrun)?;
        w.link_with_section(0, Some(&self.malloc_for_tls_replace))?;
        return w.finish();

        struct DefaultHeapSize(u32);

        impl WriteArtifact for DefaultHeapSize {
            const SYMBOL: &'static str = "__psvitalinker_DefaultHeapSize";
            const LAYOUT: Layout = safe_pod_layout(1, PodPlaceholder::U32);

            fn write_artifact(
                &self,
                artifact: &mut faerie::Artifact,
            ) -> Result<(), faerie::ArtifactError> {
                let mut w = Self::write_pod_helper(artifact)?;
                w.u32(self.0);
                w.finish()
            }
        }
    }
}

impl WriteArtifact for MallocReplace {
    const SYMBOL: &'static str = "__psvitalinker_MallocReplace";
    const LAYOUT: Layout = safe_pod_layout(13, PodPlaceholder::U32);

    fn write_artifact(&self, artifact: &mut faerie::Artifact) -> Result<(), faerie::ArtifactError> {
        let mut w = Self::write_pod_helper(artifact)?;
        w.u32(Self::LAYOUT.size() as u32);
        w.u32(1); // unknown
        w.link(0, &self.malloc_init)?;
        w.link(0, &self.malloc_term)?;
        w.link(0, &self.malloc)?;
        w.link(0, &self.free)?;
        w.link(0, &self.calloc)?;
        w.link(0, &self.realloc)?;
        w.link(0, &self.memalign)?;
        w.link(0, &self.reallocalign)?;
        w.link(0, &self.malloc_stats)?;
        w.link(0, &self.malloc_stats_fast)?;
        w.link(0, &self.malloc_usable_size)?;
        w.finish()
    }
}

impl WriteArtifact for OperatorNewReplace {
    const SYMBOL: &'static str = "__psvitalinker_OperatorNewReplace";
    const LAYOUT: Layout = safe_pod_layout(10, PodPlaceholder::U32);

    fn write_artifact(&self, artifact: &mut faerie::Artifact) -> Result<(), faerie::ArtifactError> {
        let mut w = Self::write_pod_helper(artifact)?;
        w.u32(Self::LAYOUT.size() as u32);
        w.u32(1); // unknown
        w.link(0, &self.operator_new)?;
        w.link(0, &self.operator_new_nothrow)?;
        w.link(0, &self.operator_new_arr)?;
        w.link(0, &self.operator_new_arr_nothrow)?;
        w.link(0, &self.operator_delete)?;
        w.link(0, &self.operator_delete_nothrow)?;
        w.link(0, &self.operator_delete_arr)?;
        w.link(0, &self.operator_delete_arr_nothrow)?;
        w.finish()
    }
}

impl WriteArtifact for MallocForTlsReplace {
    const SYMBOL: &'static str = "__psvitalinker_MallocForTlsReplace";
    const LAYOUT: Layout = safe_pod_layout(6, PodPlaceholder::U32);

    fn write_artifact(&self, artifact: &mut faerie::Artifact) -> Result<(), faerie::ArtifactError> {
        let mut w = Self::write_pod_helper(artifact)?;
        w.u32(Self::LAYOUT.size() as u32);
        w.u32(1); // unknown
        w.link(0, &self.malloc_init_for_tls)?;
        w.link(0, &self.malloc_term_for_tls)?;
        w.link(0, &self.malloc_for_tls)?;
        w.link(0, &self.free_for_tls)?;
        w.finish()
    }
}
