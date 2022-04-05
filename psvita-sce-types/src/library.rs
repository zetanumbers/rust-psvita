use bitflags::bitflags;

bitflags! {
    /// Module type attributes
    #[repr(transparent)]
    pub struct LibraryAttribute: u16 {
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
