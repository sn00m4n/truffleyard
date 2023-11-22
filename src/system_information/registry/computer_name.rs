// SYSTEM hive
// "This stores the hostname of the system in the ComputerName Value." - SANS Windows Forensic Analysis Posters

use std::fs::File;
use std::io::{BufWriter, Read, Write};

use anyhow::anyhow;
use nt_hive::Hive;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ComputerName {
    computer_name: String,
}

pub fn get_computer_name(reg_file: &str, outpath: &str) -> anyhow::Result<()> {
    print!("Working on Computer Name: ");
    let mut buffer = Vec::new();
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let mut computers: Vec<ComputerName> = Vec::new();
    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;

    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Control\\ComputerName\\ComputerName")
        .ok_or(anyhow!(
            "Key 'ControlSet001\\Control\\ComputerName\\ComputerName' can not be found!"
        ))??;

    let computer_name = sub_key_node
        .value("ComputerName")
        .ok_or(anyhow!("Computername can not be found!"))??
        .string_data()?;

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
