use serde_json::Value;
use std::{
    fs,
    io::{BufReader, BufWriter},
};

const TARGET_NAME: &str = "armv7a-sony-psvita";

fn main() {
    // Paths to files
    let link_script_path = format!("src/target_config/{}.ld", TARGET_NAME);
    let target_json_config_path = format!("src/target_config/{}.no-link-script.json", TARGET_NAME);

    // React to changes
    for path in &[&target_json_config_path, &link_script_path] {
        println!("cargo:rerun-if-changed={}", path);
    }

    // Load
    let link_script = fs::read_to_string(link_script_path).unwrap();
    let mut target_json: Value = {
        let rdr = BufReader::new(fs::File::open(target_json_config_path).unwrap());
        serde_json::from_reader(rdr).unwrap()
    };

    // Insert `link-script`
    target_json
        .as_object_mut()
        .unwrap()
        .insert(String::from("link-script"), Value::String(link_script));

    // Save
    {
        let writer = BufWriter::new(fs::File::create(format!("{}.json", TARGET_NAME)).unwrap());
        serde_json::to_writer(writer, &target_json).unwrap();
    }
}
