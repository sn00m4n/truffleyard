use std::fs::File;
use std::io::{BufWriter, Read, Write};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use common::{convert_win_time, Lazy};
use nt_hive::Hive;
use regex::Regex;
use serde::Serialize;

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
#[allow(clippy::unwrap_used)]
static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("Disk&Ven_(?<man>.*?)&Prod_(?<titl>.*?)&Rev_(?<vers>\\S+)").unwrap());

pub fn sys_get_usbstor_data(reg_file: &str, outpath: &str) -> anyhow::Result<()> {
    print!("Working on USBSTOR: ");
    let mut buffer = Vec::new();
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;
    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Enum\\USBSTOR")
        .ok_or(anyhow!(
            "Key 'ControlSet001\\Enum\\USBSTOR' can not be found!"
        ))??;

    let sub_key_nodes = sub_key_node
        .subkeys()
        .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

    let mut usbstor_entries: Vec<UsbStorEntry> = Vec::new(); // liste mit structs erstellen

    // first subkey (the ones under "USBSTOR")
    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys?;
        let subkey = sub_key.name()?.to_string();
        let captures = RE
            .captures(&subkey)
            .ok_or(anyhow!("Captures are not okay!"))?;
        let manufacturer = captures
            .name("man")
            .ok_or(anyhow!("Manufacturer not found!"))?
            .as_str()
            .trim()
            .to_string();
        let title = captures
            .name("titl")
            .ok_or(anyhow!("Title not found!"))?
            .as_str()
            .trim()
            .to_string();
        let version = captures
            .name("vers")
            .ok_or(anyhow!("Version not found!"))?
            .as_str()
            .trim()
            .to_string();

        // subkeys under DiskVen Subkeys aka Serialnumbers
        let serial_key = sub_key
            .subkeys()
            .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;
        for serkey in serial_key {
            let serkey = serkey?;
            let friendly_name = serkey.value("FriendlyName");
            if let Some(Ok(fr)) = friendly_name {
                let serialnumber = serkey.name()?.to_string();
                let timestamp = convert_win_time(serkey.header().timestamp.get());

                // and further down
                //subkey under serial_subkey:
                let sersubkey = serkey
                    .subkeys()
                    .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                for sersub_key in sersubkey {
                    let sersub_key = sersub_key?;

                    if sersub_key.name()?.to_string().starts_with("Properties") {
                        let propsubkey = sersub_key
                            .subkeys()
                            .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                        for propertykey in propsubkey {
                            let propertykey = propertykey?;

                            if propertykey.name()?.to_string().starts_with("{83da") {
                                let property = propertykey
                                    .subkeys()
                                    .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                                for prop in property {
                                    let prop = prop?;
                                    // first connection time
                                    if prop.name()?.to_string().starts_with("0064") {
                                        let fi_timestamp =
                                            convert_win_time(prop.header().timestamp.get());

                                        let property = propertykey
                                            .subkeys()
                                            .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                                        for prop in property {
                                            let prop = prop?;
                                            // last connection time
                                            if prop.name()?.to_string().starts_with("0066") {
                                                let lc_timestamp =
                                                    convert_win_time(prop.header().timestamp.get());

                                                let property = propertykey.subkeys().ok_or(
                                                    anyhow!("Subkeys can not be unwrapped!"),
                                                )??;

                                                for prop in property {
                                                    let prop = prop?;
                                                    if prop.name()?.to_string().starts_with("0067")
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
                let serialnumber = serkey.name()?.to_string();
                let timestamp = convert_win_time(serkey.header().timestamp.get());

                // and further down
                //subkey under serial_subkey:
                let sersubkey = serkey
                    .subkeys()
                    .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                for sersub_key in sersubkey {
                    let sersub_key = sersub_key?;

                    if sersub_key.name()?.to_string().starts_with("Properties") {
                        let propsubkey = sersub_key
                            .subkeys()
                            .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                        for propertykey in propsubkey {
                            let propertykey = propertykey?;

                            if propertykey.name()?.to_string().starts_with("{83da") {
                                let property = propertykey
                                    .subkeys()
                                    .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                                for prop in property {
                                    let prop = prop?;
                                    //first connection time
                                    if prop.name()?.to_string().starts_with("0064") {
                                        let fi_timestamp =
                                            convert_win_time(prop.header().timestamp.get());

                                        let property = propertykey
                                            .subkeys()
                                            .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

                                        for prop in property {
                                            let prop = prop?;
                                            // last connection time
                                            if prop.name()?.to_string().starts_with("0066") {
                                                let lc_timestamp =
                                                    convert_win_time(prop.header().timestamp.get());

                                                let property = propertykey.subkeys().ok_or(
                                                    anyhow!("Subkeys can not be unwrapped!"),
                                                )??;

                                                for prop in property {
                                                    let prop = prop?;
                                                    if prop.name()?.to_string().starts_with("0067")
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
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(format!("{outpath}/reg_usbstor.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &usbstor_entries)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
