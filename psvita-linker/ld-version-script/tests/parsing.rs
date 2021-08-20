use std::borrow::Cow;

use ld_version_script::{GlobPattern, TrivialVersionScript};

#[test]
fn parse_basic() {
    const VERSION_SCRIPT_SRC: &str = r#"
    {
        global:
        local:
            *;
    };
    "#;

    let got: TrivialVersionScript = VERSION_SCRIPT_SRC.parse().unwrap();
    let mut expect = TrivialVersionScript::new();

    // global
    {}

    // local
    {
        expect.local.insert(GlobPattern {
            has_suffix: true,
            prefix: Cow::Borrowed(&[]),
        });
    }

    assert_eq!(got, expect);
}

#[test]
fn parse_intersecting() {
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
    let mut expect = TrivialVersionScript::new();

    // global
    {
        expect.global.insert(GlobPattern {
            has_suffix: false,
            prefix: Cow::Borrowed("a".as_bytes()),
        });
        expect.global.insert(GlobPattern {
            has_suffix: false,
            prefix: Cow::Borrowed("ab".as_bytes()),
        });
        expect.global.insert(GlobPattern {
            has_suffix: true,
            prefix: Cow::Borrowed("abc".as_bytes()),
        });
        expect.global.insert(GlobPattern {
            has_suffix: false,
            prefix: Cow::Borrowed("abd".as_bytes()),
        });
    }

    // local
    {
        expect.local.insert(GlobPattern {
            has_suffix: true,
            prefix: Cow::Borrowed(&[]),
        });
    }

    assert_eq!(got, expect);
}
