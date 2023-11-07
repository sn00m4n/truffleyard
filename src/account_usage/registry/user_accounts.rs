// "Identify both local and domain accounts with interactive logins to the system"- SANS Windows Forensic Analysis Poster, User Accounts
// SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList

use std::fs::File;
use std::io::Read;

use chrono::{DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct ProfileListEntry {
    timestamp: DateTime<Utc>,
    sid: String, //using key name instead of Sid:RegBinary for now
    profile_image_path: String,
}

// retrieve data about users
pub fn get_profile_list(reg_file: &String, out_json: String) -> Result<(), Error> {
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let mut profile_list_list: Vec<ProfileListEntry> = Vec::new();

    let sub_key_node = root_key_node
        .subpath("Microsoft\\Windows NT\\CurrentVersion\\ProfileList")
        .unwrap()
        .unwrap();

    let sub_key_nodes = sub_key_node.subkeys().unwrap().unwrap();

    for sub_keys in sub_key_nodes {
        let sub_keys = sub_keys.unwrap();
        let sid = sub_keys.name().unwrap().to_string();
        let timestamp = convert_win_time(sub_keys.header().timestamp.get());
        let profile_image_path = sub_keys
            .value("ProfileImagePath")
            .unwrap()
            .unwrap()
            .string_data()
            .unwrap();
        let profile_list_entry = ProfileListEntry {
            timestamp,
            sid,
            profile_image_path,
        };
        profile_list_list.push(profile_list_entry);
    }

    if profile_list_list.is_empty() {
        println!("Nothing to do :(");
        return Ok(());
    }

    write_json_lines(out_json, &profile_list_list).expect("failed to write .json");
    Ok(())
}