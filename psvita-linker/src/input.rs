use ld_compat_args::{Argument, ZKeyword};
use ld_version_script::TrivialVersionScript;
use once_cell::unsync::OnceCell;
use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct LdInput {
    pub input_files: Vec<InputFile>,
    pub library_paths: Vec<PathBuf>,
    pub libraries: Vec<InputLibrary>,
    pub output_file: PathBuf,
    pub output_options: OutputOptions,
    pub eh_frame_header: bool,
    pub z_keywords: Vec<ZKeyword>,
}

#[derive(Debug)]
pub enum OutputOptions {
    Executable {
        pic: bool,
    },
    Shared {
        version_script: Option<TrivialVersionScript>,
    },
}

#[derive(Debug)]
pub struct InputFile {
    pub path: PathBuf,
    pub gc_sections: bool,
    pub whole_archive: bool,
}

#[derive(Debug)]
pub struct InputLibrary {
    pub lib: ld_compat_args::Library,
    pub only_static: bool,
    pub gc_sections: bool,
    pub whole_archive: bool,
}

impl LdInput {
    pub fn from_args() -> Self {
        let args = ld_compat_args::args();
        let args = args.map_iter(env::args().skip(1));

        let mut input_files = Vec::new();
        let mut library_paths = Vec::new();

        let mut libraries = Vec::new();
        let mut only_static = false;
        let mut gc_sections = false;
        let mut whole_archive = false;

        let output_file = OnceCell::new();
        let version_script = OnceCell::new();
        let mut pie = false;
        let mut shared = false;
        let mut eh_frame_header = false;
        let mut z_keywords = Vec::new();

        for arg in args {
            match arg {
                Argument::AsNeeded(_) => (),
                Argument::BDynamic => only_static = false,
                Argument::BStatic => only_static = true,
                Argument::EhFrameHdr => eh_frame_header = true,
                Argument::GcSections(p) => gc_sections = p,
                Argument::InputFile(path) => input_files.push(InputFile {
                    path,
                    gc_sections,
                    whole_archive,
                }),
                Argument::Library(lib) => libraries.push(InputLibrary {
                    lib,
                    only_static,
                    gc_sections,
                    whole_archive,
                }),
                Argument::LibraryPath(p) => library_paths.push(p),
                Argument::Output(o) => output_file.set(o).expect("output file specified two times"),
                Argument::PicExecutable => pie = true,
                Argument::Shared => shared = true,
                Argument::VersionScript(path) => {
                    let text = fs::read_to_string(&path).expect("cannot read version script");
                    version_script
                        .set(TrivialVersionScript::pretty_parse(&text))
                        .expect("specified multiple version scripts");
                }
                Argument::WholeArchive(w) => whole_archive = w,
                Argument::Z(z) => z_keywords.push(z),
            }
        }

        let output_options = match (pie, shared, version_script.into_inner()) {
            (pic, false, None) => OutputOptions::Executable { pic },
            (false, true, vs) => OutputOptions::Shared { version_script: vs },
            _ => panic!("cannot infer type of output file"),
        };

        let output_file = output_file
            .into_inner()
            .unwrap_or_else(|| PathBuf::from("a.out"));

        LdInput {
            input_files,
            library_paths,
            libraries,
            output_file,
            output_options,
            eh_frame_header,
            z_keywords,
        }
    }
}

impl Default for OutputOptions {
    fn default() -> Self {
        OutputOptions::Executable { pic: false }
    }
}
