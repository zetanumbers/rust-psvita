use bytemuck::{Pod, Zeroable};

pub mod module_exports;
pub mod module_info;
pub mod nid;

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

pub type USize = u32;
