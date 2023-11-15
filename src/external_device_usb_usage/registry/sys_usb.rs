use std::fs::{read_to_string, File};
use std::io::{BufWriter, Read, Write};

use chrono::{DateTime, Utc};
use common::{convert_to_hex, convert_to_int, convert_win_time, VendorList};
use nt_hive::Hive;
use serde::Serialize;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct UsbEntry {
    vid: String,
    pid: String,
    vendorname: String,
    productname: String,
    serial_number: String,
    parentidprefix: String,
    friendly_name: String,
    location_information: String,
    time_stamp: DateTime<Utc>,
}

pub fn sys_get_usb_data(
    reg_file: &str,
    vidpid_json_path: &str,
    outpath: &str,
) -> Result<(), Error> {
    print!("Working on USB: ");
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();
    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Enum\\USB")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut usb_entries: Vec<UsbEntry> = Vec::new(); // list to save structs

    let data = read_to_string(vidpid_json_path).unwrap();
    let vendors: VendorList = serde_json::from_str(&data).unwrap();

    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys.unwrap();
        let subkey = sub_key.name().unwrap().to_string();

        if subkey.starts_with("VID_") {
            let vid = subkey.split_at(4).1.split_at(4).0;
            let pid = subkey.split_at(13).1.split_at(4).0;

            let vid = convert_to_int(vid).unwrap();
            let pid = convert_to_int(pid).unwrap();

            let serial_subkey = sub_key.subkeys().unwrap().unwrap();
            for subsubkey in serial_subkey {
                let subsubkey = subsubkey.unwrap();
                let serialnumber = subsubkey.name().unwrap().to_string();
                let timestamp = convert_win_time(subsubkey.header().timestamp.get());
                let parentidpref = subsubkey.value("ParentIdPrefix");
                let location_info = subsubkey.value("LocationInformation");
                if let Some(Ok(parentid)) = parentidpref {
                    if let Some(Ok(location)) = location_info {
                        let friendlyname = subsubkey.value("FriendlyName");
                        if let Some(Ok(friendly)) = friendlyname {
                            if let Some(vendor) = vendors.get(&vid) {
                                if let Some(device) = vendor.devices.get(&pid) {
                                    let hex_pid = convert_to_hex(pid);
                                    let hex_vid = convert_to_hex(vid);
                                    let dev = UsbEntry {
                                        vid: hex_vid,
                                        pid: hex_pid,
                                        vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                        productname: device.name.clone().unwrap_or("".to_string()),
                                        serial_number: serialnumber,
                                        parentidprefix: parentid
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        friendly_name: friendly
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        location_information: location
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        time_stamp: timestamp,
                                    };
                                    usb_entries.push(dev);
                                } else {
                                    let hex_pid = convert_to_hex(pid);
                                    let hex_vid = convert_to_hex(vid);
                                    let dev = UsbEntry {
                                        vid: hex_vid,
                                        pid: hex_pid,
                                        vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                        productname: "".to_string(),
                                        serial_number: serialnumber,
                                        parentidprefix: parentid
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        friendly_name: friendly
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        location_information: location
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        time_stamp: timestamp,
                                    };
                                    usb_entries.push(dev);
                                }
                            } else {
                                let hex_pid = convert_to_hex(pid);
                                let hex_vid = convert_to_hex(vid);
                                let dev = UsbEntry {
                                    vid: hex_vid,
                                    pid: hex_pid,
                                    vendorname: "".to_string(),
                                    productname: "".to_string(),
                                    serial_number: serialnumber,
                                    parentidprefix: parentid
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    friendly_name: friendly.string_data().unwrap_or("".to_string()),
                                    location_information: location
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    time_stamp: timestamp,
                                };
                                usb_entries.push(dev);
                            }
                        } else if let Some(vendor) = vendors.get(&vid) {
                            if let Some(device) = vendor.devices.get(&pid) {
                                let hex_pid = convert_to_hex(pid);
                                let hex_vid = convert_to_hex(vid);
                                let dev = UsbEntry {
                                    vid: hex_vid,
                                    pid: hex_pid,
                                    vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                    productname: device.name.clone().unwrap_or("".to_string()),
                                    serial_number: serialnumber,
                                    parentidprefix: parentid
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    friendly_name: "".to_string(),
                                    location_information: location
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    time_stamp: timestamp,
                                };
                                usb_entries.push(dev);
                            } else {
                                let hex_pid = convert_to_hex(pid);
                                let hex_vid = convert_to_hex(vid);
                                let dev = UsbEntry {
                                    vid: hex_vid,
                                    pid: hex_pid,
                                    vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                    productname: "".to_string(),
                                    serial_number: serialnumber,
                                    parentidprefix: parentid
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    friendly_name: "".to_string(),
                                    location_information: location
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    time_stamp: timestamp,
                                };
                                usb_entries.push(dev);
                            }
                        } else {
                            let hex_pid = convert_to_hex(pid);
                            let hex_vid = convert_to_hex(vid);
                            let dev = UsbEntry {
                                vid: hex_vid,
                                pid: hex_pid,
                                vendorname: "".to_string(),
                                productname: "".to_string(),
                                serial_number: serialnumber,
                                parentidprefix: parentid.string_data().unwrap_or("".to_string()),
                                friendly_name: "".to_string(),
                                location_information: location
                                    .string_data()
                                    .unwrap_or("".to_string()),
                                time_stamp: timestamp,
                            };
                            usb_entries.push(dev);
                        }
                    } else {
                        let friendlyname = subsubkey.value("FriendlyName");
                        if let Some(Ok(friendly)) = friendlyname {
                            if let Some(vendor) = vendors.get(&vid) {
                                if let Some(device) = vendor.devices.get(&pid) {
                                    let hex_pid = convert_to_hex(pid);
                                    let hex_vid = convert_to_hex(vid);
                                    let dev = UsbEntry {
                                        vid: hex_vid,
                                        pid: hex_pid,
                                        vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                        productname: device.name.clone().unwrap_or("".to_string()),
                                        serial_number: serialnumber,
                                        parentidprefix: parentid
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        friendly_name: friendly
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        location_information: "".to_string(),
                                        time_stamp: timestamp,
                                    };
                                    usb_entries.push(dev);
                                } else {
                                    let hex_pid = convert_to_hex(pid);
                                    let hex_vid = convert_to_hex(vid);
                                    let dev = UsbEntry {
                                        vid: hex_vid,
                                        pid: hex_pid,
                                        vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                        productname: parentid
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        serial_number: serialnumber,
                                        parentidprefix: "".to_string(),
                                        friendly_name: friendly
                                            .string_data()
                                            .unwrap_or("".to_string()),
                                        location_information: "".to_string(),
                                        time_stamp: timestamp,
                                    };
                                    usb_entries.push(dev);
                                }
                            } else {
                                let hex_pid = convert_to_hex(pid);
                                let hex_vid = convert_to_hex(vid);
                                let dev = UsbEntry {
                                    vid: hex_vid,
                                    pid: hex_pid,
                                    vendorname: "".to_string(),
                                    productname: "".to_string(),
                                    serial_number: serialnumber,
                                    parentidprefix: parentid
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    friendly_name: friendly.string_data().unwrap_or("".to_string()),
                                    location_information: "".to_string(),
                                    time_stamp: timestamp,
                                };
                                usb_entries.push(dev);
                            }
                        } else if let Some(vendor) = vendors.get(&vid) {
                            if let Some(device) = vendor.devices.get(&pid) {
                                let hex_pid = convert_to_hex(pid);
                                let hex_vid = convert_to_hex(vid);
                                let dev = UsbEntry {
                                    vid: hex_vid,
                                    pid: hex_pid,
                                    vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                    productname: device.name.clone().unwrap_or("".to_string()),
                                    serial_number: serialnumber,
                                    parentidprefix: parentid
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    friendly_name: "".to_string(),
                                    location_information: "".to_string(),
                                    time_stamp: timestamp,
                                };
                                usb_entries.push(dev);
                            } else {
                                let hex_pid = convert_to_hex(pid);
                                let hex_vid = convert_to_hex(vid);
                                let dev = UsbEntry {
                                    vid: hex_vid,
                                    pid: hex_pid,
                                    vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                    productname: "".to_string(),
                                    serial_number: serialnumber,
                                    parentidprefix: parentid
                                        .string_data()
                                        .unwrap_or("".to_string()),
                                    friendly_name: "".to_string(),
                                    location_information: "".to_string(),
                                    time_stamp: timestamp,
                                };
                                usb_entries.push(dev);
                            }
                        } else {
                            let hex_pid = convert_to_hex(pid);
                            let hex_vid = convert_to_hex(vid);
                            let dev = UsbEntry {
                                vid: hex_vid,
                                pid: hex_pid,
                                vendorname: "".to_string(),
                                productname: "".to_string(),
                                serial_number: serialnumber,
                                parentidprefix: parentid.string_data().unwrap_or("".to_string()),
                                friendly_name: "".to_string(),
                                location_information: "".to_string(),
                                time_stamp: timestamp,
                            };
                            usb_entries.push(dev);
                        }
                    }
                }
            }
        }
    }
    if usb_entries.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(format!("{outpath}/reg_usb.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &usb_entries)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
