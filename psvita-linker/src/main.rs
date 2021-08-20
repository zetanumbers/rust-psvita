use log::debug;
use psvita_linker::input::Input;

fn main() {
    pretty_env_logger::init_custom_env("PSVITA_LINKER_LOG");

    let input = Input::from_args();
    debug!("Parsed input as: {:#?}", &input);
    todo!()
}
