use crate::Sym;
use faerie::{Artifact, ArtifactError};
use std::{alloc::Layout, ffi::CStr, ops::Range};

pub trait WriteArtifact {
    const SYMBOL: &'static str;
    const LAYOUT: Layout;
    fn write_artifact(&self, artifact: &mut Artifact) -> Result<(), ArtifactError>;
    fn write_pod_helper(artifact: &mut Artifact) -> Result<WritePodHelper, ArtifactError> {
        WritePodHelper::new(artifact, Self::SYMBOL.into(), Self::LAYOUT)
    }
}

pub fn create_artifact() -> Artifact {
    use target_lexicon::*;

    faerie::ArtifactBuilder::new(Triple {
        architecture: Architecture::Arm(ArmArchitecture::Armv7a),
        vendor: Vendor::Custom(CustomVendor::Static("sony")),
        operating_system: OperatingSystem::Unknown,
        environment: Environment::Eabihf,
        binary_format: BinaryFormat::Elf,
    })
    .library(true)
    .name("sce_module_info".to_owned())
    .finish()
}

pub struct WritePodHelper<'a, 'b> {
    artifact: &'a mut Artifact,
    symbol: &'b Sym,
    layout: Layout,
    buffer: Vec<u8>,
}

impl<'a, 'b> WritePodHelper<'a, 'b> {
    pub fn new(
        artifact: &'a mut Artifact,
        symbol: &'b Sym,
        layout: Layout,
    ) -> Result<Self, ArtifactError> {
        assert_eq!(layout.size() % layout.align(), 0);
        artifact.declare(
            &symbol,
            faerie::Decl::Defined(faerie::artifact::DefinedDecl::Section(
                faerie::SectionDecl::new(faerie::SectionKind::Text)
                    .with_datatype(faerie::DataType::Bytes)
                    .with_align(Some(layout.align() as u64)),
            )),
        )?;
        Ok(Self {
            buffer: Vec::with_capacity(layout.size()),
            artifact,
            symbol,
            layout,
        })
    }

    pub fn u8(&mut self, v: u8) {
        assert!(self.buffer.len() + 1 <= self.layout.size());
        self.buffer.push(v);
    }

    pub fn u8_slice(&mut self, v: &[u8]) {
        assert!(self.buffer.len() + v.len() <= self.layout.size());
        self.buffer.extend_from_slice(v);
    }

    pub fn u16(&mut self, v: u16) {
        assert!(self.layout.align() >= 2);
        assert_eq!(self.buffer.len() % 2, 0);
        assert!(self.buffer.len() + 2 <= self.layout.size());
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u32(&mut self, v: u32) {
        assert!(self.layout.align() >= 4);
        assert_eq!(self.buffer.len() % 4, 0);
        assert!(self.buffer.len() + 4 <= self.layout.size());
        self.buffer.extend_from_slice(&v.to_le_bytes());
    }

    pub fn link<T>(&mut self, v: u32, sym: &Option<T>) -> Result<(), ArtifactError>
    where
        T: AsRef<Sym>,
    {
        if let Some(sym) = sym {
            self.artifact.link(faerie::Link {
                from: &self.symbol,
                to: sym.as_ref(),
                at: self.buffer.len() as u64,
            })?;
        }
        self.u32(v);
        Ok(())
    }

    pub fn link_range<T>(&mut self, v: u32, range: &Option<Range<T>>) -> Result<(), ArtifactError>
    where
        T: AsRef<Sym>,
    {
        self.link(v, &range.as_ref().map(|r| &r.start))?;
        self.link(v, &range.as_ref().map(|r| &r.end))
    }

    pub fn link_with_cstr<S>(
        &mut self,
        v: u32,
        sym: &Sym,
        data: &Option<S>,
    ) -> Result<(), ArtifactError>
    where
        S: AsRef<CStr>,
    {
        let available = data.is_some();
        if let Some(data) = data {
            self.artifact.declare_with(
                sym,
                faerie::Decl::cstring(),
                data.as_ref().to_bytes_with_nul().into(),
            )?;
        }
        self.link(v, &available.then(|| sym))
    }

    pub fn link_with_section<T>(&mut self, v: u32, data: Option<&T>) -> Result<(), ArtifactError>
    where
        T: WriteArtifact,
    {
        let symbol = data
            .map(|data| data.write_artifact(self.artifact).map(|()| T::SYMBOL))
            .transpose()?;
        self.link(v, &symbol)
    }

    pub fn finish(self) -> Result<(), ArtifactError> {
        assert_eq!(self.buffer.len(), self.layout.size());
        self.artifact.define(self.symbol, self.buffer)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PodPlaceholder {
    U8,
    U16,
    U32,
}

pub const fn safe_pod_layout(count: usize, elem: PodPlaceholder) -> Layout {
    let align = match elem {
        PodPlaceholder::U8 => 1,
        PodPlaceholder::U16 => 2,
        PodPlaceholder::U32 => 4,
    };
    let size = align * count;
    unsafe { Layout::from_size_align_unchecked(size, align) }
}
