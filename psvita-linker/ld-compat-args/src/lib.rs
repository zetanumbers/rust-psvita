use std::path::PathBuf;
use thiserror::Error;

#[remain::sorted]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Argument {
    AsNeeded(bool),
    BDynamic,
    BStatic,
    EhFrameHdr,
    GcSections(bool),
    InputFile(PathBuf),
    Library(Library),
    LibraryPath(PathBuf),
    Output(PathBuf),
    PicExecutable,
    Shared,
    VersionScript(PathBuf),
    WholeArchive(bool),
    Z(ZKeyword),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Library {
    Name(String),
    File(String),
}

impl From<String> for Library {
    fn from(mut val: String) -> Self {
        if val.starts_with(':') {
            val.replace_range(..1, "");
            Library::File(val)
        } else {
            Library::Name(val)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZKeyword {
    Noexecstack,
}

impl From<String> for ZKeyword {
    fn from(s: String) -> Self {
        s.parse().expect("parsing `-z` option")
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseZKeywordError {
    #[error("unknown -z keyword")]
    Unknown,
}

impl std::str::FromStr for ZKeyword {
    type Err = ParseZKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noexecstack" => Ok(ZKeyword::Noexecstack),
            _ => Err(ParseZKeywordError::Unknown),
        }
    }
}

pub fn args() -> compat_args::Args<self::Argument> {
    use Argument::*;

    let mut args = compat_args::Args::new();
    let compat_args::Args {
        plain_handler,
        flags,
        shorts,
        longs,
    } = &mut args;

    *plain_handler = Some(Box::new(|s| InputFile(s.into())));

    flags.insert("--as-needed", || AsNeeded(true));
    flags.insert("--no-as-needed", || AsNeeded(false));

    let handler = || BDynamic;
    flags.insert("-Bdynamic", handler);
    flags.insert("-dy", handler);
    flags.insert("-call_shared", handler);

    let handler = || BStatic;
    flags.insert("-Bstatic", handler);
    flags.insert("-dn", handler);
    flags.insert("-non_shared", handler);
    flags.insert("-static", handler);

    flags.insert("--eh-frame-hdr", || EhFrameHdr);

    flags.insert("--gc-sections", || GcSections(true));
    flags.insert("--no-gc-sections", || GcSections(false));

    let handler = || PicExecutable;
    flags.insert("-pie", handler);
    flags.insert("--pic-executable", handler);

    let handler = || Shared;
    flags.insert("-shared", handler);
    flags.insert("-Bshareable", handler);

    flags.insert("--whole-archive", || WholeArchive(true));
    flags.insert("--no-whole-archive", || WholeArchive(false));

    let handler = Library;
    shorts.insert(*b"-l", handler);
    longs.insert("--library", handler);

    let handler = LibraryPath;
    shorts.insert(*b"-L", handler);
    longs.insert("--library-path", handler);

    let handler = Output;
    shorts.insert(*b"-o", handler);
    longs.insert("--output", handler);

    longs.insert("--version-script", VersionScript);

    shorts.insert(*b"-z", Z);

    args
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_args() {
        let input_args = ["--version-script=/tmp/rustchCaNJl/list", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/psvita_dylib_example.420ogeym4143ooxn.rcgu.o", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/psvita_dylib_example.2jdmp6cbjz7q76p1.rcgu.o", "--as-needed", "-L", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps", "-L", "/home/zeta0/rust-psvita/examples/target/debug/deps", "-L", "/home/zeta0/.vitasdk/arm-vita-eabi/lib", "-L", "/home/zeta0/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/armv7a-sony-psvita/lib", "-Bstatic", "--whole-archive", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/librustc_std_workspace_core-d03d8b57bcedbd94.rlib", "--no-whole-archive", "--whole-archive", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/libcore-82cbd9ce51306110.rlib", "--no-whole-archive", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/libcompiler_builtins-6d33f699b0f7befc.rlib", "-Bdynamic", "--eh-frame-hdr", "-znoexecstack", "-L", "/home/zeta0/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/armv7a-sony-psvita/lib", "-o", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/libpsvita_dylib_example.vso", "-shared"];
        let args: Vec<_> = super::args()
            .map_iter(input_args.iter().map(|&s| s.to_owned()))
            .collect();
        eprintln!("{:#?}", args);
    }
}
