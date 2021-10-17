#[test]
fn henkaku_nid_example() {
    assert_eq!(
        0xEEDA2E54,
        psvita_sce_types::generate_nid(b"sceDisplayGetFrameBuf")
    );
}
