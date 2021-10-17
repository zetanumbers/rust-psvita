use psvita_sce_types::nid::Nid;

#[test]
fn henkaku_nid_example() {
    assert_eq!(Nid(0xEEDA2E54), Nid::from_bytes(b"sceDisplayGetFrameBuf"));
}
