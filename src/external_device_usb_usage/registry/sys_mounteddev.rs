use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

use anyhow::anyhow;
use common::read_extended_ascii;
use encoding_rs::UTF_16LE;
use nt_hive::Hive;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

// drive letter and volume name from System Hive

//regex 1
static RE: Lazy<Regex> = Lazy::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(".*?Ven_(?<man>.*?)&Prod_(?<prod>.*?)&Rev_(?<vers>.*?)#(?<ser>.*?)#(?<gid>.*?\\S*)")
        .unwrap()
});

//regex 2
#[allow(clippy::unwrap_used)]
static RB: Lazy<Regex> = Lazy::new(|| {
    Regex::new(".*?Ven_(?<man>.*?)&Prod_(?<prod>.*?)#(?<ser>.*?)#(?<gid>.*?\\S*)").unwrap()
});

#[derive(Debug, Serialize)]
pub struct MountedDevice {
    device_name: String, // key value name
    device_data: String,
    vendorname: String,
    productname: String,
    revision: String,
    serial: String,
    guid: String,
}

pub fn sys_get_mounteddev_data(reg_file: &str, outpath: &str) -> anyhow::Result<()> {
    print!("Working on Mounted Devices: ");
    let mut buffer = Vec::new();
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;
    let sub_key_node = root_key_node
        .subpath("MountedDevices")
        .ok_or(anyhow!("Key 'MountedDevices' can not be found!"))??;
    let mut mounted_devices: Vec<MountedDevice> = Vec::new();

    for values in sub_key_node.values() {
        let values = values?;
        for value in values {
            let value = value?;
            let value_name = value.name()?;
            //println!("{}", value_name);
            let value_data = value.data()?.into_vec()?;
            let n = 2;
            let result: Vec<_> = value_data.iter().skip(n - 1).step_by(n).copied().collect();
            //println!("{:?}", result);

            let another_number = result.iter().find(|b| **b != 0);
            //println!("{:?}", another_number);

            if another_number.is_some() {
                let ha = read_extended_ascii(&value_data, 0, value_data.len());
                if let Some(str) = ha {
                    //println!("{str}");
                    let device = MountedDevice {
                        device_name: value_name.to_string(),
                        device_data: str,
                        vendorname: "".to_string(),
                        productname: "".to_string(),
                        revision: "".to_string(),
                        serial: "".to_string(),
                        guid: "".to_string(),
                    };
                    mounted_devices.push(device);
                }
            } else {
                let (string, _encodingzeugs, _invalid_chars) = UTF_16LE.decode(&value_data);
                let string = string.to_string();
                if string.contains("&Rev_") {
                    let capture = RE
                        .captures(&string)
                        .ok_or(anyhow!("Captures are not okay!"))?;
                    let vendorname = capture
                        .name("man")
                        .ok_or(anyhow!("Vendorname not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let productname = capture
                        .name("prod")
                        .ok_or(anyhow!("Productname not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let revision = capture
                        .name("vers")
                        .ok_or(anyhow!("Revision not found!"))?
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
                    //  println!("{vendorname}, {productname}, {revision}, {serial}, {guid}");
                    let device = MountedDevice {
                        device_name: value_name.to_string(),
                        device_data: string.clone(),
                        vendorname,
                        productname,
                        revision,
                        serial,
                        guid,
                    };

                    mounted_devices.push(device);
                }
                if !string.contains("&Rev") {
                    let capture = RB
                        .captures(&string)
                        .ok_or(anyhow!("Captures are not okay!"))?;
                    let vendorname = capture
                        .name("man")
                        .ok_or(anyhow!("Vendorname not found!"))?
                        .as_str()
                        .trim()
                        .to_string();
                    let productname = capture
                        .name("prod")
                        .ok_or(anyhow!("Productname not found!"))?
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
                    //println!("{vendorname}, {productname}, {serial}, {guid}");
                    let device = MountedDevice {
                        device_name: value_name.to_string(),
                        device_data: string.clone(),
                        vendorname,
                        productname,
                        revision: "".to_string(),
                        serial,
                        guid,
                    };
                    mounted_devices.push(device);
                }
            }
        }
    }
    if mounted_devices.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(format!("{outpath}/reg_mounted_devices.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &mounted_devices)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
