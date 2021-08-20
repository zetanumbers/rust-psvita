use ld_version_script::TrivialVersionScript;

#[test]
fn is_match() {
    const VERSION_SCRIPT_SRC: &str = r#"
    {
        global:
            a;
            ab;
            abc;
            abc*;
            abcd;
            abd;
        local:
            *;
    };
    "#;

    let got: TrivialVersionScript = VERSION_SCRIPT_SRC.parse().unwrap();
    
    let is_match_global = |s: &str| got.global.is_match(s.as_bytes());

    assert!(!is_match_global(""));
    assert!(is_match_global("a"));
    assert!(is_match_global("ab"));
    assert!(!is_match_global("ac"));
    assert!(is_match_global("abc"));
    assert!(is_match_global("abceeee"));
}
