use std::fs::File;
use std::io::Read;

use chrono::{DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct UsbStorEntry {
    time_stamp: DateTime<Utc>,
    manufacturer: String,
    title: String,
    version: String,
    serial_number: String,
    device_name: String,
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
    last_removed: DateTime<Utc>,
}

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("Disk&Ven_(?<man>.*?)&Prod_(?<titl>.*?)&Rev_(?<vers>\\S+)").unwrap());

pub fn sys_get_usbstor_data(reg_file: &String, outfile: String) -> Result<(), Error> {
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();
    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Enum\\USBSTOR")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut usbstor_entries: Vec<UsbStorEntry> = Vec::new(); // liste mit structs erstellen

    // first subkey (the ones under "USBSTOR")
    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys.unwrap();
        let subkey = sub_key.name().unwrap().to_string();
        let captures = RE.captures(&subkey).unwrap();
        let manufacturer = captures.name("man").unwrap().as_str().trim().to_string();
        let title = captures.name("titl").unwrap().as_str().trim().to_string();
        let version = captures.name("vers").unwrap().as_str().trim().to_string();

        // subkeys under DiskVen Subkeys aka Serialnumbers
        let serial_key = sub_key.subkeys().unwrap().unwrap();
        for serkey in serial_key {
            let serkey = serkey.unwrap();
            let friendly_name = serkey.value("FriendlyName");
            if let Some(Ok(fr)) = friendly_name {
                let serialnumber = serkey.name().unwrap().to_string();
                let timestamp = convert_win_time(serkey.header().timestamp.get());

                // and further down
                //subkey under serial_subkey:
                let sersubkey = serkey.subkeys().unwrap().unwrap();

                for sersub_key in sersubkey {
                    let sersub_key = sersub_key.unwrap();

                    if sersub_key
                        .name()
                        .unwrap()
                        .to_string()
                        .starts_with("Properties")
                    {
                        let propsubkey = sersub_key.subkeys().unwrap().unwrap();

                        for propertykey in propsubkey {
                            let propertykey = propertykey.unwrap();

                            if propertykey.name().unwrap().to_string().starts_with("{83da") {
                                let property = propertykey.subkeys().unwrap().unwrap();

                                for prop in property {
                                    let prop = prop.unwrap();
                                    // first connection time
                                    if prop.name().unwrap().to_string().starts_with("0064") {
                                        let fi_timestamp =
                                            convert_win_time(prop.header().timestamp.get());

                                        let property = propertykey.subkeys().unwrap().unwrap();

                                        for prop in property {
                                            let prop = prop.unwrap();
                                            // last connection time
                                            if prop.name().unwrap().to_string().starts_with("0066")
                                            {
                                                let lc_timestamp =
                                                    convert_win_time(prop.header().timestamp.get());

                                                let property =
                                                    propertykey.subkeys().unwrap().unwrap();

                                                for prop in property {
                                                    let prop = prop.unwrap();
                                                    if prop
                                                        .name()
                                                        .unwrap()
                                                        .to_string()
                                                        .starts_with("0067")
                                                    // last removal time
                                                    {
                                                        let lr_timestamp = convert_win_time(
                                                            prop.header().timestamp.get(),
                                                        );

                                                        let usbstor = UsbStorEntry {
                                                            time_stamp: timestamp,
                                                            manufacturer: manufacturer
                                                                .as_str()
                                                                .trim()
                                                                .to_string(),
                                                            title: title
                                                                .as_str()
                                                                .trim()
                                                                .to_string(),
                                                            version: version
                                                                .as_str()
                                                                .trim()
                                                                .to_string(),
                                                            serial_number: serialnumber.clone(),
                                                            device_name: fr
                                                                .string_data()
                                                                .unwrap_or("".to_string()),
                                                            first_connected: fi_timestamp,
                                                            last_connected: lc_timestamp,
                                                            last_removed: lr_timestamp,
                                                        };
                                                        usbstor_entries.push(usbstor);
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
            } else {
                let serialnumber = serkey.name().unwrap().to_string();
                let timestamp = convert_win_time(serkey.header().timestamp.get());

                // and further down
                //subkey under serial_subkey:
                let sersubkey = serkey.subkeys().unwrap().unwrap();

                for sersub_key in sersubkey {
                    let sersub_key = sersub_key.unwrap();

                    if sersub_key
                        .name()
                        .unwrap()
                        .to_string()
                        .starts_with("Properties")
                    {
                        let propsubkey = sersub_key.subkeys().unwrap().unwrap();

                        for propertykey in propsubkey {
                            let propertykey = propertykey.unwrap();

                            if propertykey.name().unwrap().to_string().starts_with("{83da") {
                                let property = propertykey.subkeys().unwrap().unwrap();

                                for prop in property {
                                    let prop = prop.unwrap();
                                    //first connection time
                                    if prop.name().unwrap().to_string().starts_with("0064") {
                                        let fi_timestamp =
                                            convert_win_time(prop.header().timestamp.get());

                                        let property = propertykey.subkeys().unwrap().unwrap();

                                        for prop in property {
                                            let prop = prop.unwrap();
                                            // last connection time
                                            if prop.name().unwrap().to_string().starts_with("0066")
                                            {
                                                let lc_timestamp =
                                                    convert_win_time(prop.header().timestamp.get());

                                                let property =
                                                    propertykey.subkeys().unwrap().unwrap();

                                                for prop in property {
                                                    let prop = prop.unwrap();
                                                    if prop
                                                        .name()
                                                        .unwrap()
                                                        .to_string()
                                                        .starts_with("0067")
                                                    //last removal time
                                                    {
                                                        let lr_timestamp = convert_win_time(
                                                            prop.header().timestamp.get(),
                                                        );

                                                        let usbstor = UsbStorEntry {
                                                            time_stamp: timestamp,
                                                            manufacturer: manufacturer
                                                                .as_str()
                                                                .trim()
                                                                .to_string(),
                                                            title: title
                                                                .as_str()
                                                                .trim()
                                                                .to_string(),
                                                            version: version
                                                                .as_str()
                                                                .trim()
                                                                .to_string(),
                                                            serial_number: serialnumber.clone(),
                                                            device_name: "".to_string(),
                                                            first_connected: fi_timestamp,
                                                            last_connected: lc_timestamp,
                                                            last_removed: lr_timestamp,
                                                        };
                                                        usbstor_entries.push(usbstor);
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
        }
    }
    if usbstor_entries.is_empty() {
        println!("Nothing to do.");
        return Ok(());
    }
    write_json_lines(outfile, &usbstor_entries).expect("failed to write .json");
    Ok(())
}
