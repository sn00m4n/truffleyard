// SYSTEM hive
// "This determines system type, version, build number and installation dates for previous updates." - SANS Windows Forensic Analysis Poster

use std::fs::File;
use std::io::{BufWriter, Read, Write};

use chrono::{DateTime, NaiveDateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct SourceOSEntry {
    current_build_number: String,
    edition_id: String,
    installation_type: String,
    install_date: NaiveDateTime,
    install_time: DateTime<Utc>,
    path_name: String,
    product_id: String,
    product_name: String,
    registered_organization: String,
    registered_owner: String,
    software_type: String,
}

pub fn get_os_updates(reg_file: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on previous OS Versions: ");
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let sub_key_node = root_key_node.subpath("Setup").unwrap().unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut sourceos_entries: Vec<SourceOSEntry> = Vec::new();

    for subkeys in sub_key_nodes {
        let subkey = subkeys.unwrap();
        let subname = subkey.name().unwrap().to_string();
        if subname.starts_with("Source OS") {
            let current_build_number = subkey
                .value("CurrentBuildNumber")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let edition_id = subkey
                .value("EditionID")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let installation_type = subkey
                .value("InstallationType")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let install_d = subkey
                .value("InstallDate")
                .unwrap()
                .unwrap()
                .dword_data()
                .unwrap() as i64;
            let install_date = NaiveDateTime::from_timestamp_opt(install_d, 0).unwrap();
            let install_t = subkey
                .value("InstallTime")
                .unwrap()
                .unwrap()
                .qword_data()
                .unwrap();
            let install_time = convert_win_time(install_t);
            let path_name = subkey
                .value("PathName")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let product_id = subkey
                .value("ProductID")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let product_name = subkey
                .value("ProductName")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let registered_organization = subkey
                .value("RegisteredOrganization")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let registered_owner = subkey
                .value("RegisteredOwner")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let software_type = subkey
                .value("SoftwareType")
                .unwrap()
                .unwrap()
                .string_data()
                .unwrap_or("".to_string());
            let source_os_entry = SourceOSEntry {
                current_build_number,
                edition_id,
                installation_type,
                install_date,
                install_time,
                path_name,
                product_id,
                product_name,
                registered_organization,
                registered_owner,
                software_type,
            };

            sourceos_entries.push(source_os_entry);
        }
    }
    if sourceos_entries.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }

    let file = File::create(outpath)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &sourceos_entries)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
