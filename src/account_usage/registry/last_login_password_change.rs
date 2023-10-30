// "The SAM hive maintains a list of local accounts and associated configuration information" - SANS Windows Forensic Analysis Poster, Last Login and Password Changes

use std::fs::File;
use std::io::Read;

use chrono::{Date, DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
pub struct AccountDetails {
    rid: String,
    last_login_time: DateTime<Utc>,
    last_pw_change: DateTime<Utc>,
    // login_counts: u32,
    // group_membership: String,
    account_creation_time: DateTime<Utc>,
}

pub fn get_account_details(reg_file: &String, out_json: String) -> Result<(), Error> {
    Ok(())
}
