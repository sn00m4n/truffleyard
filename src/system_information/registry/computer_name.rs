use std::fs::File;
use std::io::Read;

use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct ComputerName {
    computer_name: String,
}

pub fn get_computer_name(reg_file: &String, out_json: String) -> Result<(), Error> {
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let mut computers: Vec<ComputerName> = Vec::new();
    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Control\\ComputerName\\ComputerName")
        .unwrap()
        .unwrap();

    let computer_name = sub_key_node
        .value("ComputerName")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap();

    let computername = ComputerName { computer_name };
    computers.push(computername);
    if computers.is_empty() {
        println!("Nothing to do.");
        return Ok(());
    }
    write_json_lines(out_json, &computers).expect("failed to write json :(");
    Ok(())
}
