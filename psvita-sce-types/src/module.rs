use crate::{
    helpers::{safe_pod_layout, PodPlaceholder, WriteArtifact, WritePodHelper},
    Symbol,
};
use arrayvec::ArrayVec;
use bitflags::bitflags;
use faerie::ArtifactError;
use std::{alloc::Layout, num::NonZeroU8, ops::Range};

#[derive(Clone, Debug)]
pub struct Module {
    pub attributes: ModuleAttributes,
    pub privilege_level: ModulePrivilegeLevel,
    /// usually set to 1
    pub major_version: u8,
    /// usually set to 1
    pub minor_version: u8,
    pub name: ArrayVec<NonZeroU8, MODULE_NAME_MAX_LEN>,
    /// TODO
    pub exports: Vec<()>,
    /// TODO
    pub imports: Vec<()>,
    /// It was wrongly named module NID. It is a sort of hash to ensure integrity and versioning.
    pub debug_fingerprint: u32,
    pub tls: Option<ModuleTlsInfo>,
    pub start_entry: Option<Symbol>,
    pub stop_entry: Option<Symbol>,
    pub arm_exidx: Option<Range<Symbol>>,
    pub arm_extab: Option<Range<Symbol>>,
}

pub const MODULE_NAME_MAX_LEN: usize = 26;

bitflags! {
    /// Module type attributes
    pub struct ModuleAttributes: u8 {
        const CANT_STOP = 0x01;
        const EXCLUSIVE_LOAD = 0x02;
        const EXCLUSIVE_START = 0x04;
    }

    /// Module Privilege Levels - These levels define the permissions a module can have.
    pub struct ModulePrivilegeLevel: u8 {
        /// Lowest permission
        const USER                 = 0x00;
        /// MS modeul. POPS/Demo.
        const MS                   = 0x02;
        /// USB WLAN module. Gamesharin.
        const USBWLAN              = 0x04;
        /// Application module
        const APP                  = 0x06;
        /// VSH module
        const VSH                  = 0x08;
        /// Kernel module. Highest permission.
        const KERNEL               = 0x10;
        /// The module uses KIRK's memlmd resident library
        const KIRK_MEMLMD_LIB      = 0x20;
        /// The module uses KIRK's semaphore resident library
        const KIRK_SEMAPHORE_LIB   = 0x40;
    }
}

#[derive(Clone, Debug)]
pub struct ModuleTlsInfo {
    /// Offset to start of TLS (Thread Local Storage)
    pub start: Symbol,
    /// Certainly equals (tls_end - tls_start)
    pub size: u32,
    /// Certainly equals (tls_initialized_data_end - tls_start)
    pub initialized_size: u32,
}

impl WriteArtifact for Module {
    const SYMBOL: &'static str = "__psvitalinker_sceModuleInfo";
    const LAYOUT: Layout = safe_pod_layout(23, PodPlaceholder::U32);

    fn write_artifact(&self, artifact: &mut faerie::Artifact) -> Result<(), ArtifactError> {
        let mut w = Self::write_pod_helper(artifact)?;
        w.u8(self.attributes.bits());
        w.u8(self.privilege_level.bits());
        w.u8(self.major_version);
        w.u8(self.minor_version);
        w.u8_slice(&self.packed_name());
        w.u8(6); // SceModuleInfo version
        w.u32(0); // gp_value
        w.u32(0); // TODO exports
        w.u32(0); // TODO imports
        w.u32(self.debug_fingerprint);
        write_tls(&mut w, self.tls.as_ref())?;
        w.link(u32::MAX, &self.start_entry)?;
        w.link(u32::MAX, &self.stop_entry)?;
        w.link_range(0, &self.arm_exidx)?;
        w.link_range(0, &self.arm_extab)?;
        w.finish()
    }
}

impl Module {
    pub fn generate_artifact(&self) -> Result<faerie::Artifact, ArtifactError> {
        let mut artifact = crate::helpers::create_artifact();
        self.write_artifact(&mut artifact)?;
        Ok(artifact)
    }

    fn packed_name(&self) -> [u8; MODULE_NAME_MAX_LEN + 1] {
        let mut name = [0; MODULE_NAME_MAX_LEN + 1];
        name.iter_mut()
            .zip(&self.name)
            .for_each(|(dst, src)| *dst = src.get());
        name
    }
}

fn write_tls(
    w: &mut WritePodHelper<'_, '_>,
    tls: Option<&ModuleTlsInfo>,
) -> Result<(), ArtifactError> {
    w.link(0, &tls.map(|tls| &tls.start))?;
    let (size, init_size) = match tls {
        Some(tls) => (tls.size, tls.initialized_size),
        None => (0, 0),
    };
    w.u32(size);
    w.u32(init_size);
    Ok(())
}
