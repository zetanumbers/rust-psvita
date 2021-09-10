use std::{env, path::Path, process::Command};

#[test]
fn minimal() {
    build("examples/minimal".as_ref());
}

#[test]
fn dynamic_linking() {
    build("examples/dynamic_linking".as_ref());
}

fn build(package: &Path) {
    let cargo_program = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let cargo_command = || Command::new(&cargo_program);

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    {
        let dir = dbg!(workspace.join("examples"));
        let status = cargo_command()
            .arg("clean")
            .current_dir(dir)
            .status()
            .unwrap();
        assert!(status.success());
    }

    let linker_dir = workspace.join("target").join("debug");
    {
        let mut cmd = cargo_command();
        cmd.args(&["build", "-ppsvita-linker"]);
        eprintln!("{:?}", cmd);
        let status = cmd.status().unwrap();
        assert!(status.success());
    }

    let path = format!(
        "{}:{}",
        env::var("PATH").unwrap(),
        linker_dir.to_str().unwrap()
    );

    let status = cargo_command()
        .args(&["run", "-pcargo-psvita", "--"])
        .args(&["--title=TEST0000", "--"])
        .arg("--manifest-path")
        .arg(workspace.join(package).join("Cargo.toml").to_str().unwrap())
        .arg("-v")
        .env("PATH", path)
        .env("RUSTC_LOG", "rustc_codegen_ssa::back::link=trace")
        .env("PSVITA_LINKER_LOG", "debug")
        .status()
        .unwrap();

    assert!(status.success());
}
