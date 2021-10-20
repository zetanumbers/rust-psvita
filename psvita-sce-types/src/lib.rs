use core::marker::PhantomData;

use bytemuck::{Pod, Zeroable};

pub mod module_exports;
pub mod module_info;
pub mod nid;

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
