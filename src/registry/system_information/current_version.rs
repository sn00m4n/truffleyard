use std::fs::File;
use std::io::Read;

use chrono::{DateTime, NaiveDateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

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

pub fn get_current_os_version(reg_file: &String, out_json: String) {
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let sub_key_node = root_key_node
        .subpath("Microsoft\\Windows NT\\CurrentVersion")
        .unwrap()
        .unwrap();

    let mut os_entry: Vec<SourceOSEntry> = Vec::new();

    let current_build_number = sub_key_node
        .value("CurrentBuildNumber")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let edition_id = sub_key_node
        .value("EditionID")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let installation_type = sub_key_node
        .value("InstallationType")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let install_d = sub_key_node
        .value("InstallDate")
        .unwrap()
        .unwrap()
        .dword_data()
        .unwrap() as i64;
    let install_date = NaiveDateTime::from_timestamp_opt(install_d, 0).unwrap();
    let install_t = sub_key_node
        .value("InstallTime")
        .unwrap()
        .unwrap()
        .qword_data()
        .unwrap();
    let install_time = convert_win_time(install_t);
    let path_name = sub_key_node
        .value("PathName")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let product_id = sub_key_node
        .value("ProductID")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let product_name = sub_key_node
        .value("ProductName")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let registered_organization = sub_key_node
        .value("RegisteredOrganization")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let registered_owner = sub_key_node
        .value("RegisteredOwner")
        .unwrap()
        .unwrap()
        .string_data()
        .unwrap_or("".to_string());
    let software_type = sub_key_node
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

    os_entry.push(source_os_entry);

    write_json_lines(&out_json, &os_entry).expect("failed to write .json");
}
