mod common;

pub use common::Args as CommonArgs;
use displaydoc::Display;
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
    File(PathBuf),
}

impl From<String> for Library {
    fn from(mut val: String) -> Self {
        if val.starts_with(':') {
            val.replace_range(..1, "");
            Library::File(val.into())
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

#[derive(Display, Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseZKeywordError {
    /// unknown -z keyword
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

pub fn args() -> CommonArgs<self::Argument> {
    let mut args = CommonArgs::new(|s| Argument::InputFile(s.into()));

    macro_rules! flag {
        ($flags:expr, $produced:ident) => {
            flag_impl(|| Argument::$produced, &$flags, &mut args);
        };
        ($flags:expr, $no_flags:expr, $produced:ident) => {
            flag_impl(|| Argument::$produced(true), &$flags, &mut args);
            flag_impl(|| Argument::$produced(false), &$no_flags, &mut args);
        };
    }

    macro_rules! implicit_optional {
        () => {
            None
        };
        ($v:expr) => {
            Some($v)
        };
    }

    macro_rules! option {
        ($($short:expr)?, $($long:expr)?, $parser:ident) => {
            option_impl(
                |s| Argument::$parser(s.into()),
                implicit_optional!($(*$short)*),
                implicit_optional!($($long)*),
                &mut args,
            );
        };
    }

    pub fn flag_impl(
        producer: fn() -> Argument,
        flags: &[&'static str],
        args: &mut CommonArgs<Argument>,
    ) {
        flags
            .iter()
            .copied()
            .for_each(|f| args.flags.insert(f, producer))
    }

    pub fn option_impl(
        parser: fn(String) -> Argument,
        short: Option<[u8; 2]>,
        long: Option<&'static str>,
        args: &mut CommonArgs<Argument>,
    ) {
        if let Some(short) = short {
            args.short.insert(short, parser);
        }
        if let Some(long) = long {
            args.long.insert(long, parser);
        }
    }

    #[rustfmt::skip]
    let () = {
        flag!(["--as-needed"]
             ,["--no-as-needed"]
             , AsNeeded
             );

        flag!(["-Bdynamic"
              ,"-dy"
              ,"-call_shared"
             ], BDynamic
             );

        flag!(["-Bstatic"
              ,"-dn"
              ,"-non_shared"
              ,"-static"
             ], BStatic
             );

        flag!(["--eh-frame-hdr"]
             , EhFrameHdr
             );

        flag!(["--gc-sections"]
             ,["--no-gc-sections"]
             , GcSections
             );

        flag!(["-pie"
              ,"--pic-executable"
             ], PicExecutable
             );

        flag!(["-shared"
              , "-Bshareable"
             ], Shared
             );

        flag!(["--whole-archive"]
             ,["--no-whole-archive"]
             , WholeArchive
             );

        option!(b"-l"
               , "--library"
               , Library
               );

        option!(b"-L"
               , "--library-path"
               , LibraryPath
               );

        option!(b"-o"
               , "--output"
               , Output
               );

        option!(
               , "--version-script"
               , VersionScript
               );

        option!(b"-z"
               ,
               , Z
               );
    };

    args
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_args() {
        let input_args = ["psvita-linker", "--version-script=/tmp/rustchCaNJl/list", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/psvita_dylib_example.420ogeym4143ooxn.rcgu.o", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/psvita_dylib_example.2jdmp6cbjz7q76p1.rcgu.o", "--as-needed", "-L", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps", "-L", "/home/zeta0/rust-psvita/examples/target/debug/deps", "-L", "/home/zeta0/.vitasdk/arm-vita-eabi/lib", "-L", "/home/zeta0/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/armv7a-sony-psvita/lib", "-Bstatic", "--whole-archive", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/librustc_std_workspace_core-d03d8b57bcedbd94.rlib", "--no-whole-archive", "--whole-archive", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/libcore-82cbd9ce51306110.rlib", "--no-whole-archive", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/libcompiler_builtins-6d33f699b0f7befc.rlib", "-Bdynamic", "--eh-frame-hdr", "-znoexecstack", "-L", "/home/zeta0/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/armv7a-sony-psvita/lib", "-o", "/home/zeta0/rust-psvita/examples/target/armv7a-sony-psvita/debug/deps/libpsvita_dylib_example.vso", "-shared"];
        let args: Vec<_> = super::args()
            .map_iter(input_args.iter().map(|&s| s.to_owned()))
            .collect();
        println!("{:#?}", args);
    }
}
