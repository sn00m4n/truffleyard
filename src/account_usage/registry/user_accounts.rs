// "Identify both local and domain accounts with interactive logins to the system"- SANS Windows Forensic Analysis Poster, User Accounts
// SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList

use std::fs::File;
use std::io::{BufWriter, Read, Write};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ProfileListEntry {
    timestamp: DateTime<Utc>,
    sid: String, //using key name instead of Sid:RegBinary for now
    profile_image_path: String,
}

// retrieve data about users
pub fn get_profile_list(reg_file: &str, outpath: &str) -> anyhow::Result<()> {
    print!("Working on User Accounts: ");
    let mut buffer = Vec::new();
    File::open(reg_file)?.read_to_end(&mut buffer)?;

    let hive = Hive::without_validation(buffer.as_ref())?;
    let root_key_node = hive.root_key_node()?;

    let mut profile_list_list: Vec<ProfileListEntry> = Vec::new();

    let sub_key_node = root_key_node
        .subpath("Microsoft\\Windows NT\\CurrentVersion\\ProfileList")
        .ok_or(anyhow!(
            "Key 'Microsoft\\Windows NT\\CurrentVersion\\ProfileList' not found!"
        ))??;

    let sub_key_nodes = sub_key_node
        .subkeys()
        .ok_or(anyhow!("Subkeys can not be unwrapped!"))??;

    for sub_keys in sub_key_nodes {
        let sub_keys = sub_keys?;
        let sid = sub_keys.name()?.to_string();
        let timestamp = convert_win_time(sub_keys.header().timestamp.get());
        let profile_image_path = sub_keys
            .value("ProfileImagePath")
            .ok_or(anyhow!("Key 'Profile Image Path' not found!"))??
            .string_data()?;
        let profile_list_entry = ProfileListEntry {
            timestamp,
            sid,
            profile_image_path,
        };
        profile_list_list.push(profile_list_entry);
    }

    if profile_list_list.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }

    let file = File::create(format!("{outpath}/reg_useraccounts.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &profile_list_list)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
