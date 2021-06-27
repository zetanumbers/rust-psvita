use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    CargoOpt, Metadata, MetadataCommand,
};
use std::{
    collections::HashSet,
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;

mod target_config;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Space or comma separated list of features to activate
    #[structopt(long)]
    features: Vec<String>,

    /// Activate all available features
    #[structopt(long)]
    all_features: bool,

    /// Do not activate the `default` feature
    #[structopt(long)]
    no_default_features: bool,

    /// Path to Cargo.toml
    #[structopt(long, parse(from_os_str))]
    manifest_path: Option<PathBuf>,

    /// TITLEID passed to the vita-mksfoex
    #[structopt(long)]
    title: String,

    /// Arguments for cargo build
    build_args: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    let metadata = get_metadata(&opt);
    exists_or_create_dir(&metadata.target_directory);
    copy_target_configuration(&metadata.target_directory);

    let executable_elfs = build(&opt, &metadata);
    for elf in executable_elfs {
        let out_dir = elf.parent().unwrap();
        let stem = elf.file_stem().unwrap();

        generate_velf(out_dir, stem);
        generate_eboot(out_dir, stem);
        make_sfo(out_dir, stem, &opt.title);
        eprintln!("Produced vpk: {}", pack_vpk(out_dir, stem));
    }
}

fn generate_velf(out_dir: &Utf8Path, stem: &str) -> Utf8PathBuf {
    let elf = out_dir.join(format!("{}.elf", stem));
    let output = out_dir.join(format!("{}.velf", stem));

    assert!(
        Command::new(vitasdk_bin().join("vita-elf-create"))
            .args(&[&elf, &output])
            .status()
            .expect("could not execute vita-elf-create")
            .success(),
        "failed while executing vita-elf-create"
    );
    output
}

fn generate_eboot(out_dir: &Utf8Path, stem: &str) -> Utf8PathBuf {
    let velf = out_dir.join(format!("{}.velf", stem));
    let output = out_dir.join(format!("{}.eboot.bin", stem));

    assert!(
        Command::new(vitasdk_bin().join("vita-make-fself"))
            .arg("-s")
            .args(&[&velf, &output])
            .status()
            .expect("could not execute vita-make-fself")
            .success(),
        "failed while executing vita-make-fself"
    );
    output
}

fn make_sfo(out_dir: &Utf8Path, stem: &str, title: &str) -> Utf8PathBuf {
    let output = out_dir.join(format!("{}.sfo", stem));

    assert!(
        Command::new(vitasdk_bin().join("vita-mksfoex"))
            .arg(title)
            .arg(&output)
            .status()
            .expect("could not execute vita-mksfoex")
            .success(),
        "failed while executing vita-mksfoex"
    );
    output
}

fn pack_vpk(out_dir: &Utf8Path, stem: &str) -> Utf8PathBuf {
    let sfo = out_dir.join(format!("{}.sfo", stem));
    let eboot = out_dir.join(format!("{}.eboot.bin", stem));
    let output = out_dir.join(format!("{}.vpk", stem));

    assert!(
        Command::new(vitasdk_bin().join("vita-pack-vpk"))
            .arg("--sfo")
            .arg(sfo)
            .arg("--eboot")
            .arg(eboot)
            .arg(&output)
            .status()
            .expect("could not execute vita-pack-vpk")
            .success(),
        "failed while executing vita-pack-vpk"
    );
    output
}

fn vitasdk() -> PathBuf {
    PathBuf::from(env::var_os("VITASDK").expect("could not find VITASDK environment variable"))
}

fn vitasdk_bin() -> PathBuf {
    vitasdk().join("bin")
}

fn build(opt: &Opt, metadata: &Metadata) -> Vec<Utf8PathBuf> {
    let rustflags = env::var("RUSTFLAGS").unwrap_or_default();
    let rustflags = format!(
        "{} -L {}",
        rustflags,
        vitasdk()
            .join("arm-vita-eabi")
            .join("lib")
            .to_str()
            .unwrap()
    );

    let out = Command::new(env::var("CARGO").unwrap_or(String::from("cargo")))
        .arg("build")
        .args(&[
            "--target",
            metadata.target_directory.join(target_config::NAME).as_str(),
        ])
        .args(&["-Z", "build-std=core"])
        .args(&["--message-format", "json"])
        .args(&opt.features)
        .args(opt.all_features.then(|| "--all-features"))
        .args(opt.no_default_features.then(|| "--no-default-features"))
        .args(&opt.manifest_path)
        .args(&opt.build_args)
        .env("RUSTFLAGS", rustflags)
        .stderr(Stdio::inherit())
        .output()
        .expect("could not execute cargo-build");

    assert!(out.status.success(), "cargo-build failed");

    // Package ids within workspace (due to `no_deps` argument)
    let local_pkg_ids = metadata
        .packages
        .iter()
        .map(|pkg| &pkg.id)
        .collect::<HashSet<_>>();

    cargo_metadata::Message::parse_stream(out.stdout.as_slice())
        .filter_map(|msg| match msg.expect("error while parsing message") {
            cargo_metadata::Message::CompilerArtifact(a) => Some(a),
            _ => None,
        })
        .filter(|artifact| local_pkg_ids.contains(&artifact.package_id))
        .filter_map(|artifact| artifact.executable)
        .collect()
}

fn exists_or_create_dir(dir: &Utf8Path) {
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("could not create target directory");
    }
    assert!(dir.is_dir());
}

fn get_metadata(opt: &Opt) -> Metadata {
    let mut cmd = MetadataCommand::new();
    cmd.features(CargoOpt::SomeFeatures(opt.features.clone()));
    if opt.all_features {
        cmd.features(CargoOpt::AllFeatures);
    }
    if opt.no_default_features {
        cmd.features(CargoOpt::NoDefaultFeatures);
    }
    if let Some(manifest_path) = &opt.manifest_path {
        cmd.manifest_path(manifest_path);
    }
    cmd.no_deps()
        .exec()
        .expect("could not get the crate's metadata")
}

fn copy_target_configuration(target_dir: &Utf8Path) {
    fs::write(target_dir.join(target_config::NAME), target_config::CONTENT)
        .expect("could not copy target configuration to the target directory");
}
