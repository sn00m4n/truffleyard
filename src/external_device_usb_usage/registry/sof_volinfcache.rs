use std::fs::File;
use std::io::{BufWriter, Read, Write};

use chrono::{DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;

use crate::errors::Error;

//SOFTWARE HIVE
#[derive(Debug, Serialize)]
struct VicEntry {
    timestamp: DateTime<Utc>,
    drive_name: String,
    vol_label: String,
    drive_type: String,
}

pub fn sof_get_vic_data(reg_file: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on Volume Info Cache: ");
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    //VolumeInfoCache
    let sub_key_node = root_key_node
        .subpath("Microsoft\\Windows Search\\VolumeInfoCache")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    let mut vic_entries: Vec<VicEntry> = Vec::new();

    for sub_keys in sub_key_nodes {
        let sub_key = sub_keys.unwrap();

        let drivetype = sub_key.value("DriveType");
        if let Some(Ok(drive)) = drivetype {
            let drivedata = drive.dword_data().unwrap();
            // check different drive types
            // drive type 3 -> fixed disk
            if drivedata == 3 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "Fixed Disk".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "Fixed Disk".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // drive type 5 -> CDRom
            if drivedata == 5 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "CDRom".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "CDRom".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // drive type 4 -> network drive
            if drivedata == 4 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "Network Drive".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "Network Drive".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // drive type 1 -> no root directory
            if drivedata == 1 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "NoRootDirectory".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "NoRootDirectory".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // drive type 6 -> RAM disk
            if drivedata == 6 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "RAM Disk".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "RAM Disk".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // drive type 2 -> Removable storage device
            if drivedata == 2 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "Removable Storage Device".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "Removable Storage Device".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // drive type 0 -> unknown
            if drivedata == 0 {
                let drive_name = sub_key.name().unwrap().to_string();
                let timestamp = convert_win_time(sub_key.header().timestamp.get());
                let vollabel = sub_key.value("VolumeLabel");
                if let Some(Ok(vol)) = vollabel {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: vol.string_data().unwrap(),
                        drive_type: "Unknown".to_string(),
                    };
                    vic_entries.push(vicentry);
                } else {
                    let vicentry = VicEntry {
                        timestamp,
                        drive_name,
                        vol_label: "".to_string(),
                        drive_type: "Unknown".to_string(),
                    };
                    vic_entries.push(vicentry);
                }
            }

            // anything else
        } else {
            let drive_name = sub_key.name().unwrap().to_string();
            let timestamp = convert_win_time(sub_key.header().timestamp.get());
            let vollabel = sub_key.value("VolumeLabel");
            if let Some(Ok(vol)) = vollabel {
                let vicentry = VicEntry {
                    timestamp,
                    drive_name,
                    vol_label: vol.string_data().unwrap(),
                    drive_type: "".to_string(),
                };
                vic_entries.push(vicentry);
            } else {
                let vicentry = VicEntry {
                    timestamp,
                    drive_name,
                    vol_label: "".to_string(),
                    drive_type: "".to_string(),
                };
                vic_entries.push(vicentry);
            }
        }
    }

    if vic_entries.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(format!("{outpath}/reg_volume_info_cache.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &vic_entries)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
