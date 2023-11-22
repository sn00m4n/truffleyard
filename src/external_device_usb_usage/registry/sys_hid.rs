use std::fs::{read_to_string, File};
use std::io::{BufWriter, Read, Write};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use common::{convert_to_hex, convert_to_int, convert_win_time, VendorList};
use nt_hive::Hive;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HdiEntry {
    full_key_name: String,
    time_stamp: DateTime<Utc>,
    vendor_id: String,
    product_id: String,
    vendorname: String,
    productname: String,
    serialnumber: String,
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
}

pub fn sys_get_hid_data(
    reg_file: &str,
    vidpid_json_path: &str,
    outpath: &str,
) -> anyhow::Result<()> {
    print!("Working on HID: ");
    let mut buffer = Vec::new();
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;
    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Enum\\HID")
        .ok_or(anyhow!("Key 'ControlSet001\\Enum\\HID' can not be found!"))??;

    let sub_key_nodes = sub_key_node
        .subkeys()
        .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

    let mut hdi_entries: Vec<HdiEntry> = Vec::new();

    let data = read_to_string(vidpid_json_path)?;
    let vendors: VendorList = serde_json::from_str(&data)?;

    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys?;
        let subkey = sub_key.name()?.to_string();

        if subkey.starts_with("VID_") || subkey.starts_with("Vid") {
            let vid = subkey.split_at(4).1.split_at(4).0;
            let pid = subkey.split_at(13).1.split_at(4).0;

            let vid = convert_to_int(vid)?;
            let pid = convert_to_int(pid)?;

            let serial_subkey = sub_key
                .subkeys()
                .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;
            for subsubkey in serial_subkey {
                let subsubkey = subsubkey?;
                let serialnumber = subsubkey.name()?.to_string();
                let timestamp = convert_win_time(subsubkey.header().timestamp.get());

                let parentsubkey = subsubkey
                    .subkeys()
                    .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;
                for pasubkey in parentsubkey {
                    let pasubkey = pasubkey?;

                    if pasubkey.name()?.to_string().starts_with("Properties") {
                        let propsubkey = pasubkey
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
                                        let fc_timestamp =
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

                                                if let Some(vendor) = vendors.get(&vid) {
                                                    if let Some(device) = vendor.devices.get(&pid) {
                                                        let hdi_entry = HdiEntry {
                                                            full_key_name: subkey.to_string(),
                                                            time_stamp: timestamp,
                                                            vendor_id: convert_to_hex(vid),
                                                            product_id: convert_to_hex(pid),
                                                            vendorname: vendor
                                                                .name
                                                                .clone()
                                                                .unwrap_or("".to_string()),
                                                            productname: device
                                                                .name
                                                                .clone()
                                                                .unwrap_or("".to_string()),
                                                            serialnumber: serialnumber.to_string(),
                                                            first_connected: fc_timestamp,
                                                            last_connected: lc_timestamp,
                                                        };
                                                        hdi_entries.push(hdi_entry);
                                                    } else {
                                                        let hdi_entry = HdiEntry {
                                                            full_key_name: subkey.to_string(),
                                                            time_stamp: timestamp,
                                                            vendor_id: convert_to_hex(vid),
                                                            product_id: convert_to_hex(pid),
                                                            vendorname: vendor
                                                                .name
                                                                .clone()
                                                                .unwrap_or("".to_string()),
                                                            productname: "".to_string(),
                                                            serialnumber: serialnumber.to_string(),
                                                            first_connected: fc_timestamp,
                                                            last_connected: lc_timestamp,
                                                        };
                                                        hdi_entries.push(hdi_entry);
                                                    }
                                                } else {
                                                    let hdi_entry = HdiEntry {
                                                        full_key_name: subkey.to_string(),
                                                        time_stamp: timestamp,
                                                        vendor_id: convert_to_hex(vid),
                                                        product_id: convert_to_hex(pid),
                                                        vendorname: "".to_string(),
                                                        productname: "".to_string(),
                                                        serialnumber: serialnumber.to_string(),
                                                        first_connected: fc_timestamp,
                                                        last_connected: lc_timestamp,
                                                    };
                                                    hdi_entries.push(hdi_entry);
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
            let serial_subkey = sub_key
                .subkeys()
                .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;
            for subsubkey in serial_subkey {
                let subsubkey = subsubkey?;
                let serialnumber = subsubkey.name()?.to_string();
                let timestamp = convert_win_time(subsubkey.header().timestamp.get());

                let parentsubkey = subsubkey
                    .subkeys()
                    .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;
                for pasubkey in parentsubkey {
                    let pasubkey = pasubkey?;

                    if pasubkey.name()?.to_string().starts_with("Properties") {
                        let propsubkey = pasubkey
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
                                        let fc_timestamp =
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
                                                let hdi_entry = HdiEntry {
                                                    full_key_name: subkey.to_string(),
                                                    time_stamp: timestamp,
                                                    vendor_id: "".to_string(),
                                                    product_id: "".to_string(),
                                                    vendorname: "".to_string(),
                                                    productname: "".to_string(),
                                                    serialnumber: serialnumber.to_string(),
                                                    first_connected: fc_timestamp,
                                                    last_connected: lc_timestamp,
                                                };
                                                hdi_entries.push(hdi_entry);
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
    if hdi_entries.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(format!("{outpath}/reg_hid.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &hdi_entries)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
