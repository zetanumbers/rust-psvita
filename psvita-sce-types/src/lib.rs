use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use core::marker::PhantomData;

pub mod module_exports;
pub mod module_imports;
pub mod module_info;
pub mod nid;

bitflags! {
    /// Module type attributes
    #[repr(transparent)]
    #[derive(Zeroable, Pod)]
    pub struct SceLibraryAttribute: u16 {
        /// Set for main NONAME export.
        const MAIN_EXPORT = 0x8000;
        /// In kernel modules only. Allow syscall export to userland.
        const USER_IMPORTABLE = 0x4000;
        /// On PS3, it seems to indicate a non-PRX library (like "stdc" or "allocator") that comes from somewhere else (LV2?).
        const UNKNOWN_2000 = 0x2000;
        const WEAK_IMPORT = 0x8;
        const NOLINK_EXPORT = 0x4;
        /// ?kernel non-driver export?
        const WEAK_EXPORT = 0x2;
        /// Importable: Should be set unless it is the main export. ?regular export?
        const AUTO_EXPORT = 0x1;
    }
}

pub type USize = u32;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ptr<T: 'static>(pub USize, pub PhantomData<T>);

impl<T: 'static> Ptr<T> {
    pub fn new(value: USize) -> Ptr<T> {
        Ptr(value, PhantomData)
    }
}

impl<T: 'static> std::fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:8>0X}", self.0)
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Ptr<T> {}
unsafe impl<T> Zeroable for Ptr<T> {}
unsafe impl<T> Pod for Ptr<T> {}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct PtrRange<T: 'static> {
    /// Offset to top of range
    pub top: Ptr<T>,
    /// Offset to bottom of range
    pub bottom: Ptr<T>,
}

impl<T> Clone for PtrRange<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for PtrRange<T> {}
unsafe impl<T> Zeroable for PtrRange<T> {}
unsafe impl<T> Pod for PtrRange<T> {}

pub type Address = Ptr<()>;
pub type AddressRange = PtrRange<()>;
