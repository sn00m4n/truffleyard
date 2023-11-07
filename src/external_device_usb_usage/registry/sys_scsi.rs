use std::fs::File;
use std::io::Read;

use chrono::{DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct ScsiEntry {
    time_stamp: DateTime<Utc>,
    manufacturer: String,
    title: String,
    parentidprefix: String,
    device_name: String,
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
}

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("Disk&Ven_(?<man>.*?)&Prod_(?<titl>.*?\\S*)").unwrap());

pub fn sys_get_scsi_data(reg_file: &str, outpath: &str) -> Result<(), Error> {
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();
    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Enum\\SCSI")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut scsi_entries: Vec<ScsiEntry> = Vec::new();

    // subkeys under "SCSI"
    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys.unwrap();
        let subkey = sub_key.name().unwrap().to_string();
        let captures = RE.captures(&subkey).unwrap();
        let manufacturer = captures.name("man").unwrap().as_str().trim().to_string();
        let title = captures.name("titl").unwrap().as_str().trim().to_string();

        // subkeys unter DiskVen -> ParentIdPrefix
        let parentid = sub_key.subkeys().unwrap().unwrap();
        for para_key in parentid {
            let para_key = para_key.unwrap();
            let friendly_name = para_key.value("FriendlyName");
            if let Some(Ok(friendly)) = friendly_name {
                let parentidpre = para_key.name().unwrap().to_string();
                let timestamp = convert_win_time(para_key.header().timestamp.get());

                // subkeys under parentidprefix
                let parentsubkey = para_key.subkeys().unwrap().unwrap();
                for pasubkey in parentsubkey {
                    let pasubkey = pasubkey.unwrap();

                    if pasubkey
                        .name()
                        .unwrap()
                        .to_string()
                        .starts_with("Properties")
                    {
                        let propsubkey = pasubkey.subkeys().unwrap().unwrap();
                        for propertykey in propsubkey {
                            let propertykey = propertykey.unwrap();

                            if propertykey.name().unwrap().to_string().starts_with("{83da") {
                                let property = propertykey.subkeys().unwrap().unwrap();

                                for prop in property {
                                    let prop = prop.unwrap();
                                    //first connection time
                                    if prop.name().unwrap().to_string().starts_with("0064") {
                                        let fc_timestamp =
                                            convert_win_time(prop.header().timestamp.get());

                                        let property = propertykey.subkeys().unwrap().unwrap();

                                        for prop in property {
                                            let prop = prop.unwrap();
                                            // last connection time
                                            if prop.name().unwrap().to_string().starts_with("0066")
                                            {
                                                let lc_timestamp =
                                                    convert_win_time(prop.header().timestamp.get());

                                                let scsi_entry = ScsiEntry {
                                                    time_stamp: timestamp,
                                                    manufacturer: manufacturer
                                                        .as_str()
                                                        .trim()
                                                        .to_string(),
                                                    title: title.as_str().trim().to_string(),
                                                    parentidprefix: parentidpre.clone(),
                                                    device_name: friendly
                                                        .string_data()
                                                        .unwrap_or("".to_string()),
                                                    first_connected: fc_timestamp,
                                                    last_connected: lc_timestamp,
                                                };
                                                scsi_entries.push(scsi_entry);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let parentidpre = para_key.name().unwrap().to_string();
                let timestamp = convert_win_time(para_key.header().timestamp.get());

                // subkeys under parentidprefix
                let parentsubkey = para_key.subkeys().unwrap().unwrap();
                for pasubkey in parentsubkey {
                    let pasubkey = pasubkey.unwrap();

                    if pasubkey
                        .name()
                        .unwrap()
                        .to_string()
                        .starts_with("Properties")
                    {
                        let propsubkey = pasubkey.subkeys().unwrap().unwrap();
                        for propertykey in propsubkey {
                            let propertykey = propertykey.unwrap();

                            if propertykey.name().unwrap().to_string().starts_with("{83da") {
                                let property = propertykey.subkeys().unwrap().unwrap();

                                for prop in property {
                                    let prop = prop.unwrap();
                                    //first connection time
                                    if prop.name().unwrap().to_string().starts_with("0064") {
                                        let fc_timestamp =
                                            convert_win_time(prop.header().timestamp.get());

                                        let property = propertykey.subkeys().unwrap().unwrap();

                                        for prop in property {
                                            let prop = prop.unwrap();
                                            // last connection time
                                            if prop.name().unwrap().to_string().starts_with("0066")
                                            {
                                                let lc_timestamp =
                                                    convert_win_time(prop.header().timestamp.get());

                                                let scsi_entry = ScsiEntry {
                                                    time_stamp: timestamp,
                                                    manufacturer: manufacturer
                                                        .as_str()
                                                        .trim()
                                                        .to_string(),
                                                    title: title.as_str().trim().to_string(),
                                                    parentidprefix: parentidpre.clone(),
                                                    device_name: "".to_string(),
                                                    first_connected: fc_timestamp,
                                                    last_connected: lc_timestamp,
                                                };
                                                scsi_entries.push(scsi_entry);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if scsi_entries.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    write_json_lines(format!("{outpath}/reg_scsi.json"), &scsi_entries)
        .expect("failed to write .json");
    Ok(())
}
