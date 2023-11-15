// SYSTEM hive
// "This stores the hostname of the system in the ComputerName Value." - SANS Windows Forensic Analysis Posters

use std::fs::File;
use std::io::{BufWriter, Read, Write};

use nt_hive::Hive;
use serde::Serialize;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct ComputerName {
    computer_name: String,
}

pub fn get_computer_name(reg_file: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on Computer Name: ");
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
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(format!("{outpath}/reg_computer_name.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &computers)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
