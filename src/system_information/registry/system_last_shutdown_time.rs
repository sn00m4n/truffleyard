// SYSTEM hive
// "It is the last time the system was shutdown. On Windows XP, the number of shutdowns is also recorded." - SANS Windows Forensic Analysis Poster
// Note: this code does not implement the Win XP artifact (yet)

use std::fs::File;
use std::io::{BufWriter, Read, Write};

use chrono::{DateTime, Utc};
use common::convert_win_time;
use nt_hive::Hive;
use serde::Serialize;
use serde_jsonlines;
use serde_jsonlines::write_json_lines;

use crate::errors::Error;

#[derive(Debug, Serialize)]
pub struct ShutdownTime {
    shutdown_time: DateTime<Utc>,
}

pub fn get_shutdown_time(reg_file: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on Last Shutdown Time: ");
    let mut buffer = Vec::new();
    File::open(reg_file)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    let mut times: Vec<ShutdownTime> = Vec::new();
    let hive = Hive::without_validation(buffer.as_ref()).unwrap();
    let root_key_node = hive.root_key_node().unwrap();

    let sub_key_node = root_key_node
        .subpath("ControlSet001\\Control\\Windows")
        .unwrap()
        .unwrap();

    let mut sd_time = sub_key_node
        .value("ShutdownTime")
        .unwrap()
        .unwrap()
        .data()
        .unwrap()
        .into_vec()
        .unwrap();

    let [a, b, c, d, e, f, g, h] = sd_time[0..8] else {
        todo!("")
    };

    let time = u64::from_le_bytes([a, b, c, d, e, f, g, h]);

    let shutdown_time = convert_win_time(time);

    let shuttime = ShutdownTime { shutdown_time };

    times.push(shuttime);

    if times.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }
    let file = File::create(outpath)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &times)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
