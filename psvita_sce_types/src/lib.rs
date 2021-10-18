use bytemuck::{Pod, Zeroable};

pub mod nid;
pub mod sce_library_entry_table;
pub mod sce_module_info;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroable, Pod)]
pub struct Address(pub u32);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Zeroable, Pod)]
pub struct AddressRange {
    /// Offset to top of address range
    pub top: Address,
    /// Offset to bottom of address range
    pub bottom: Address,
}
