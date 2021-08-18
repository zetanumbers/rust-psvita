use sce_psvita::Nid;

#[test]
fn henkaku_example() {
    assert_eq!(Nid::from(0xEEDA2E54), Nid::from("sceDisplayGetFrameBuf"));
}
