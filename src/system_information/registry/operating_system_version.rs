// SYSTEM hive
// "This determines system type, version, build number and installation dates for previous updates." - SANS Windows Forensic Analysis Poster

use std::fs::File;
use std::io::{BufWriter, Read, Write};

use anyhow::anyhow;
use chrono::{DateTime, NaiveDateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;

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

pub fn get_os_updates(reg_file: &str, outpath: &str) -> anyhow::Result<()> {
    print!("Working on previous OS Versions: ");
    let mut buffer = Vec::new();
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;

    let sub_key_node = root_key_node
        .subpath("Setup")
        .ok_or(anyhow!("Key 'Setup' can not be found!"))??;

    let sub_key_nodes = sub_key_node
        .subkeys()
        .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

    let mut sourceos_entries: Vec<SourceOSEntry> = Vec::new();

    for subkeys in sub_key_nodes {
        let subkey = subkeys?;
        let subname = subkey.name()?.to_string();
        if subname.starts_with("Source OS") {
            let current_build_number = sub_key_node
                .value("CurrentBuildNumber")
                .ok_or(anyhow!("Current Build Number can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let edition_id = sub_key_node
                .value("EditionID")
                .ok_or(anyhow!("Edition ID can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let installation_type = sub_key_node
                .value("InstallationType")
                .ok_or(anyhow!("Installation Type can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let install_d = sub_key_node
                .value("InstallDate")
                .ok_or(anyhow!("Install Date can not be found!"))??
                .dword_data()? as i64;
            let install_date = NaiveDateTime::from_timestamp_opt(install_d, 0)
                .ok_or(anyhow!("No timestamp found!"))?;
            let install_t = sub_key_node
                .value("InstallTime")
                .ok_or(anyhow!("Install Time can not be found!"))??
                .qword_data()?;
            let install_time = convert_win_time(install_t);
            let path_name = sub_key_node
                .value("PathName")
                .ok_or(anyhow!("Path Name can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let product_id = sub_key_node
                .value("ProductID")
                .ok_or(anyhow!("Product ID can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let product_name = sub_key_node
                .value("ProductName")
                .ok_or(anyhow!("Product Name can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let registered_organization = sub_key_node
                .value("RegisteredOrganization")
                .ok_or(anyhow!("Registered Organization can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let registered_owner = sub_key_node
                .value("RegisteredOwner")
                .ok_or(anyhow!("Registered Owner can not be found!"))??
                .string_data()
                .unwrap_or("".to_string());
            let software_type = sub_key_node
                .value("SoftwareType")
                .ok_or(anyhow!("Software Type can not be found!"))??
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

    let file = File::create(format!("{outpath}/reg_old_os_versions.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &sourceos_entries)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
