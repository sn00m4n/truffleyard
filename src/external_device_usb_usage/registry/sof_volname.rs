use std::fs::{read_to_string, File};
use std::io::{BufWriter, Read, Write};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use common::{convert_to_int, convert_win_time, Lazy, VendorList};
use nt_hive::Hive;
use regex::Regex;
use serde::Serialize;
use serde_json::from_str;

// drive letter and device from SOFTWARE hive

//regex1
#[allow(clippy::unwrap_used)]
static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        ".*?#DISK&VEN_(?<man>.*?)&PROD_(?<prod>.*?)&REV_(?<vers>.*?)#(?<ser>.*?)#(?<gid>.*?\\S*)",
    )
    .unwrap()
});

//regex2

static RB: Lazy<Regex> = Lazy::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(".*?#DISK&VEN_(?<man>.*?)&PROD_(?<prod>.*?)#(?<ser>.*?)#(?<gid>.*?\\S*)").unwrap()
});

//regex3
#[allow(clippy::unwrap_used)]
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
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let data = read_to_string(vidpid_json)?;
    let vendors: VendorList = from_str(&data).context("Failed at vendorlist again")?;
    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;
    let sub_key_node = root_key_node
        .subpath("Microsoft\\Windows Portable Devices")
        .ok_or(anyhow!(
            "Key 'Microsoft\\Windows Portable Devices' can not be found!"
        ))??;

    let sub_key_nodes = sub_key_node
        .subkeys()
        .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

    let mut volnames: Vec<Device> = Vec::new();

    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys?;
        if sub_key.name()?.to_string().starts_with("Devices") {
            let subsubkey = sub_key
                .subkeys()
                .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;
            for sub in subsubkey {
                let sub = sub?;
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                if sub.name()?.to_string().starts_with("SWD#")
                    && sub.name()?.to_string().contains("&REV")
                {
                    let subname = sub.name()?.to_string();
                    let capture = RE
                        .captures(&subname)
                        .ok_or(anyhow!("Captures are not okay!"))?;
                    let manufacturer = capture
                        .name("man")
                        .ok_or(anyhow!("Manufacturer not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let title = capture
                        .name("prod")
                        .ok_or(anyhow!("Title not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let version = capture
                        .name("vers")
                        .ok_or(anyhow!("Version not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let serial = capture
                        .name("ser")
                        .ok_or(anyhow!("Serialnumber not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let guid = capture
                        .name("gid")
                        .ok_or(anyhow!("GUID not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
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
                            friendly_name: friendly.string_data()?,
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
                if sub.name()?.to_string().starts_with("SWD#")
                    && sub.name()?.to_string().contains("DISK&VEN")
                {
                    let subname = sub.name()?.to_string();

                    let capture = RB
                        .captures(&subname)
                        .ok_or(anyhow!("Captures are not okay!"))?;
                    let manufacturer = capture
                        .name("man")
                        .ok_or(anyhow!("Manufacturer not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let title = capture
                        .name("prod")
                        .ok_or(anyhow!("Title not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let serial = capture
                        .name("ser")
                        .ok_or(anyhow!("Serialnumber not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let guid = capture
                        .name("gid")
                        .ok_or(anyhow!("GUID not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
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
                            friendly_name: friendly.string_data()?,
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
                if sub.name()?.to_string().starts_with("SWD#")
                    && !sub.name()?.to_string().contains("&REV")
                    && !sub.name()?.to_string().contains("DISK&VEN")
                {
                    let subname = sub.name()?.to_string();

                    let capture = ICE
                        .captures(&subname)
                        .ok_or(anyhow!("Captures are not okay!"))?;
                    let guid = capture
                        .name("gid")
                        .ok_or(anyhow!("GUID not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
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
                            friendly_name: friendly.string_data()?,
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

                if sub.name()?.to_string().starts_with("USB#") {
                    let subname2 = sub.name()?.to_string();

                    let vid = sub.name()?.to_string();
                    let svid = vid.split_at(8).1.split_at(4).0;

                    let pid = sub.name()?.to_string();
                    let spid = pid.split_at(17).1.split_at(4).0;

                    let ivid = convert_to_int(svid)?;
                    let ipid = convert_to_int(spid)?;

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
    let file = File::create(format!("{outpath}/reg_volume_name.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &volnames)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
