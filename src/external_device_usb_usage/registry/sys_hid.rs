use std::fs::{read_to_string, File};
use std::io::Read;

use chrono::{DateTime, Utc};
use common::{convert_to_hex, convert_to_int, convert_win_time, VendorList};
use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

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
) -> Result<(), Error> {
    print!("Working on HID: ");
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();
    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Enum\\HID")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut hdi_entries: Vec<HdiEntry> = Vec::new();

    let data = read_to_string(vidpid_json_path).unwrap();
    let vendors: VendorList = serde_json::from_str(&data).unwrap();

    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys.unwrap();
        let subkey = sub_key.name().unwrap().to_string();

        if subkey.starts_with("VID_") || subkey.starts_with("Vid") {
            let vid = subkey.split_at(4).1.split_at(4).0;
            let pid = subkey.split_at(13).1.split_at(4).0;

            let vid = convert_to_int(vid).unwrap();
            let pid = convert_to_int(pid).unwrap();

            let serial_subkey = sub_key.subkeys().unwrap().unwrap();
            for subsubkey in serial_subkey {
                let subsubkey = subsubkey.unwrap();
                let serialnumber = subsubkey.name().unwrap().to_string();
                let timestamp = convert_win_time(subsubkey.header().timestamp.get());

                let parentsubkey = subsubkey.subkeys().unwrap().unwrap();
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
            let serial_subkey = sub_key.subkeys().unwrap().unwrap();
            for subsubkey in serial_subkey {
                let subsubkey = subsubkey.unwrap();
                let serialnumber = subsubkey.name().unwrap().to_string();
                let timestamp = convert_win_time(subsubkey.header().timestamp.get());

                let parentsubkey = subsubkey.subkeys().unwrap().unwrap();
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
    write_json_lines(format!("{outpath}/reg_hid.json"), &hdi_entries)
        .expect("failed to write .json!");
    println!("Done here!");
    Ok(())
}
