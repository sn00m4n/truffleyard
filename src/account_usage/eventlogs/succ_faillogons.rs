// "profile account creation, attempted logons and account usage" - SANS Poster Windows Forensics, Successful/Failed Logons
// "Logon Events provide very specific information regarding the nature of account authorization on a system. In addition
// to date, time, username, hostname, and success/failure status of a logon, Logon Events also enable
// us to determine by exactly what means a logon was attempted." - SANS Poster Windows Forensics, Logon Event Types

// successful and failed logons with description

use std::fs::File;
use std::io::{BufWriter, Write};

use chrono::{DateTime, Utc};
use common::{parse_evtx, Event, Name, OuterName};
use serde::Serialize;
use serde_json::Value;
use serde_jsonlines::write_json_lines;
use xmltojson::to_json;
use {serde_xml_rs, xmltojson};

use crate::errors::Error;

// Struct for Logon Events
#[derive(Debug, Serialize)]
struct LogonEntry {
    event_record_id: u64,
    event_id: u32,
    logon_type: String,
    timestamp: DateTime<Utc>,
    description: String,
    data: Value,
}

// function to get all the data (might refactor)
pub fn sec_evtx_logons_data(input: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on Logons: ");
    let mut parser = parse_evtx(input).unwrap();
    let mut logons_list: Vec<LogonEntry> = Vec::new();
    for record in parser.records() {
        let record = record.unwrap();

        let event: Event = serde_xml_rs::from_str(&record.data).unwrap();
        let event_id = event.system.event_id;
        let logon_type = OuterName::Known(Name::LogonType);

        // Logon Event ID 4624 -> successful logon
        if event_id == 4624 {
            for data in event.event_data.unwrap().events {
                if data.name == Some(logon_type.clone()) {
                    let logontype = data.value.unwrap();
                    // sort by logon types
                    // Logon via console
                    if logontype.eq("2") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Logon via console".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // network logon
                    if logontype.eq("3") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Network Logon".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // batch logon
                    if logontype.eq("4") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Batch Logon".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // windows service logon
                    if logontype.eq("5") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Windows Service Logon".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // credentials used to unlock screen, rdp session reconnect
                    if logontype.eq("7") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Credentials used to unlock screen, RDP session reconnect"
                                .to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // network logon sending credentials (cleartext)
                    if logontype.eq("8") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Network Logon sending credentials (cleartext)"
                                .to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // different credentials used than logged on user
                    if logontype.eq("9") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Different credentials used than logged on user"
                                .to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // remote interactive logon (RDP)
                    if logontype.eq("10") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Remote interactive logon (RDP)".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // cached credentials used to login
                    if logontype.eq("11") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Cached credentials used to login".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // cached remote interactive (similar to type 10)
                    if logontype.eq("12") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Cached remote interactive".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                    // cached unlock (similar to type 7)
                    if logontype.eq("13") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let logon_entry = LogonEntry {
                            event_record_id: record.event_record_id,
                            event_id,
                            logon_type: logontype.clone(),
                            timestamp: record.timestamp,
                            description: "Cached unlock".to_string(),
                            data: json_data,
                        };
                        logons_list.push(logon_entry);
                    }
                }
            }
        }
        // failed logon
        if event_id == 4625 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let logon_entry = LogonEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "Failed Logon".to_string(),
                data: json_data,
            };
            logons_list.push(logon_entry);
        }
        // successful logoff
        if event_id == 4634 || event_id == 4647 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let logon_entry = LogonEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "Successful Logoff".to_string(),
                data: json_data,
            };
            logons_list.push(logon_entry);
        }
        // logon using explicit credentials (runas)
        if event_id == 4648 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let logon_entry = LogonEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "Logon using explicit credentials (runas)".to_string(),
                data: json_data,
            };
            logons_list.push(logon_entry);
        }
        // account logon with superuser rights (administrator)
        if event_id == 4672 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let logon_entry = LogonEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "account logon with superuser rights (administrator)".to_string(),
                data: json_data,
            };
            logons_list.push(logon_entry);
        }
        // an account was created
        if event_id == 4720 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let logon_entry = LogonEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "an account was created".to_string(),
                data: json_data,
            };
            logons_list.push(logon_entry);
        }
    }

    // check if list is empty
    if logons_list.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }

    // write output in ndjson
    let file = File::create(outpath)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &logons_list)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
