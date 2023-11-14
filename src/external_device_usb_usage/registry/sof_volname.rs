use std::fs;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Read, Write};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use common::{convert_to_int, convert_win_time, VendorList};
use nt_hive::Hive;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_json::from_str;
use serde_jsonlines::write_json_lines;

//use crate::errors::Error;
// drive letter and device from SOFTWARE hive

//regional express
static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        ".*?#DISK&VEN_(?<man>.*?)&PROD_(?<prod>.*?)&REV_(?<vers>.*?)#(?<ser>.*?)#(?<gid>.*?\\S*)",
    )
    .unwrap()
});

//regional bahn
static RB: Lazy<Regex> = Lazy::new(|| {
    Regex::new(".*?#DISK&VEN_(?<man>.*?)&PROD_(?<prod>.*?)#(?<ser>.*?)#(?<gid>.*?\\S*)").unwrap()
});

//inter city express
static ICE: Lazy<Regex> = Lazy::new(|| Regex::new(".*?UM#(?<gid>.*?)#\\S*").unwrap());
#[derive(Debug, Serialize)]
struct Device {
    full_key_name: String,
    time_stamp: DateTime<Utc>,
    vendorname: String,
    productname: String,
    version: String,
    serialnumber: String,
    guid: String,
    friendly_name: String,
}

pub fn sof_get_device_data(reg_file: &str, vidpid_json: &str, outpath: &str) -> Result<()> {
    print!("Working on Volume Names: ");
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let data = read_to_string(vidpid_json).unwrap();
    let vendors: VendorList = from_str(&data).context("Failed at vendorlist again")?;
    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();
    let sub_key_node = root_key_node
        .subpath("Microsoft\\Windows Portable Devices")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut volnames: Vec<Device> = Vec::new();

    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys.unwrap();
        if sub_key.name().unwrap().to_string().starts_with("Devices") {
            let subsubkey = sub_key.subkeys().unwrap().unwrap();
            for sub in subsubkey {
                let sub = sub.unwrap();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                if sub.name().unwrap().to_string().starts_with("SWD#")
                    && sub.name().unwrap().to_string().contains("&REV")
                {
                    let subname = sub.name().unwrap().to_string();
                    let capture = RE.captures(&subname).unwrap();
                    let manufacturer = capture.name("man").unwrap().as_str().trim().to_string();
                    let title = capture.name("prod").unwrap().as_str().trim().to_string();
                    let version = capture.name("vers").unwrap().as_str().trim().to_string();
                    let serial = capture.name("ser").unwrap().as_str().trim().to_string();
                    let guid = capture.name("gid").unwrap().as_str().trim().to_string();
                    let friendly_name = sub.value("FriendlyName");
                    if let Some(Ok(friendly)) = friendly_name {
                        let dev = Device {
                            full_key_name: subname,
                            time_stamp: timestamp,
                            vendorname: manufacturer,
                            productname: title,
                            version,
                            serialnumber: serial,
                            guid,
                            friendly_name: friendly.string_data().unwrap(),
                        };
                        volnames.push(dev);
                    } else {
                        let dev = Device {
                            full_key_name: subname,
                            time_stamp: timestamp,
                            vendorname: manufacturer,
                            productname: title,
                            version,
                            serialnumber: serial,
                            guid,
                            friendly_name: "".to_string(),
                        };
                        volnames.push(dev);
                    }
                }
                if sub.name().unwrap().to_string().starts_with("SWD#")
                    && sub.name().unwrap().to_string().contains("DISK&VEN")
                {
                    let subname = sub.name().unwrap().to_string();

                    let capture = RB.captures(&subname).unwrap();
                    let manufacturer = capture.name("man").unwrap().as_str().trim().to_string();
                    let title = capture.name("prod").unwrap().as_str().trim().to_string();
                    let serial = capture.name("ser").unwrap().as_str().trim().to_string();
                    let guid = capture.name("gid").unwrap().as_str().trim().to_string();
                    let friendly_name = sub.value("FriendlyName");
                    if let Some(Ok(friendly)) = friendly_name {
                        let dev = Device {
                            full_key_name: subname,
                            time_stamp: timestamp,
                            vendorname: manufacturer,
                            productname: title,
                            version: "".to_string(),
                            serialnumber: serial,
                            guid,
                            friendly_name: friendly.string_data().unwrap(),
                        };
                        volnames.push(dev);
                    } else {
                        let dev = Device {
                            full_key_name: subname,
                            time_stamp: timestamp,
                            vendorname: manufacturer,
                            productname: title,
                            version: "".to_string(),
                            serialnumber: serial,
                            guid,
                            friendly_name: "".to_string(),
                        };
                        volnames.push(dev);
                    }
                }
                if sub.name().unwrap().to_string().starts_with("SWD#")
                    && !sub.name().unwrap().to_string().contains("&REV")
                    && !sub.name().unwrap().to_string().contains("DISK&VEN")
                {
                    let subname = sub.name().unwrap().to_string();

                    let capture = ICE.captures(&subname).unwrap();
                    let guid = capture.name("gid").unwrap().as_str().trim().to_string();
                    let friendly_name = sub.value("FriendlyName");
                    if let Some(Ok(friendly)) = friendly_name {
                        let dev = Device {
                            full_key_name: subname,
                            time_stamp: timestamp,
                            vendorname: "".to_string(),
                            productname: "".to_string(),
                            version: "".to_string(),
                            serialnumber: "".to_string(),
                            guid,
                            friendly_name: friendly.string_data().unwrap(),
                        };
                        volnames.push(dev);
                    } else {
                        let dev = Device {
                            full_key_name: subname,
                            time_stamp: timestamp,
                            vendorname: "".to_string(),
                            productname: "".to_string(),
                            version: "".to_string(),
                            serialnumber: "".to_string(),
                            guid,
                            friendly_name: "".to_string(),
                        };
                        volnames.push(dev);
                    }
                }

                if sub.name().unwrap().to_string().starts_with("USB#") {
                    let subname2 = sub.name().unwrap().to_string();

                    let vid = sub.name().unwrap().to_string();
                    let svid = vid.split_at(8).1.split_at(4).0;

                    let pid = sub.name().unwrap().to_string();
                    let spid = pid.split_at(17).1.split_at(4).0;

                    let ivid = convert_to_int(svid).unwrap();
                    let ipid = convert_to_int(spid).unwrap();

                    let friendly_name = sub.value("FriendlyName");
                    if let Some(Ok(friendly)) = friendly_name {
                        if let Some(vendor) = vendors.get(&ivid) {
                            if let Some(device) = vendor.devices.get(&ipid) {
                                let dev = Device {
                                    full_key_name: subname2,
                                    time_stamp: timestamp,
                                    vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                    productname: device.name.clone().unwrap_or("".to_string()),
                                    version: "".to_string(),
                                    serialnumber: "".to_string(),
                                    guid: "".to_string(),
                                    friendly_name: friendly.string_data().unwrap_or("".to_string()),
                                };
                                volnames.push(dev);
                            } else {
                                let dev = Device {
                                    full_key_name: subname2,
                                    time_stamp: timestamp,
                                    vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                    productname: "".to_string(),
                                    version: "".to_string(),
                                    serialnumber: "".to_string(),
                                    guid: "".to_string(),
                                    friendly_name: friendly.string_data().unwrap_or("".to_string()),
                                };
                                volnames.push(dev);
                            }
                        } else {
                            let dev = Device {
                                full_key_name: subname2,
                                time_stamp: timestamp,
                                vendorname: "".to_string(),
                                productname: "".to_string(),
                                version: "".to_string(),
                                serialnumber: "".to_string(),
                                guid: "".to_string(),
                                friendly_name: friendly.string_data().unwrap_or("".to_string()),
                            };
                            volnames.push(dev);
                        }
                    } else if let Some(vendor) = vendors.get(&ivid) {
                        if let Some(device) = vendor.devices.get(&ipid) {
                            let dev = Device {
                                full_key_name: subname2,
                                time_stamp: timestamp,
                                vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                productname: device.name.clone().unwrap_or("".to_string()),
                                version: "".to_string(),
                                serialnumber: "".to_string(),
                                guid: "".to_string(),
                                friendly_name: "".to_string(),
                            };
                            volnames.push(dev);
                        } else {
                            let dev = Device {
                                full_key_name: subname2,
                                time_stamp: timestamp,
                                vendorname: vendor.name.clone().unwrap_or("".to_string()),
                                productname: "".to_string(),
                                version: "".to_string(),
                                serialnumber: "".to_string(),
                                guid: "".to_string(),
                                friendly_name: "".to_string(),
                            };
                            volnames.push(dev);
                        }
                    } else {
                        let dev = Device {
                            full_key_name: subname2,
                            time_stamp: timestamp,
                            vendorname: "".to_string(),
                            productname: "".to_string(),
                            version: "".to_string(),
                            serialnumber: "".to_string(),
                            guid: "".to_string(),
                            friendly_name: "".to_string(),
                        };
                        volnames.push(dev);
                    }
                }
            }
        }
    }
    if volnames.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(outpath)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &volnames)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
