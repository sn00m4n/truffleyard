use std::fs::File;
use std::io::Read;

use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

pub fn shimcache_data(reg_file: &String, out_json: String) {
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Control\\Session Manager\\AppCompatCache")
        .unwrap()
        .unwrap();

    let values = sub_key_node.values().unwrap().unwrap();
    for value in values {
        let value = value.unwrap();
        let name = value.name().unwrap();
        let typi = value.data_type().unwrap();

        println!("{}: {:?}", name, typi);
    }
}
