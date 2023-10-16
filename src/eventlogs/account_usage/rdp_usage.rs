// "track rdp logons and session reconnections to target machines" - SANS Poster Windows Forensics

use chrono::{DateTime, Utc};
use common::{parse_evtx, Event, Name, OuterName};
use serde::{Deserializer, Serialize};
use serde_json::Value;
use serde_jsonlines::write_json_lines;
use xmltojson::to_json;
use {serde_xml_rs, xmltojson};

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct RDPEventEntry {
    event_record_id: u64,
    event_id: u32,
    logon_type: String,
    timestamp: DateTime<Utc>,
    description: String,
    data: Value,
}

pub fn evtx_rdp_usage_data(input: &String, outfile: String) -> Result<(), Error> {
    let mut parser = parse_evtx(input).unwrap();
    let mut rdp_usage_list: Vec<RDPEventEntry> = Vec::new();

    for record in parser.records() {
        let record = record.unwrap();
        let event: Event = serde_xml_rs::from_str(&record.data).unwrap();

        let event_id = event.system.event_id;
        let logon_type = OuterName::Known(Name::LogonType);
        println!("{event_id}");
        if event_id == 4624 {
            for data in event.event_data.unwrap().events {
                if data.name == logon_type {
                    let test = data.value.unwrap();
                    if test.eq("10") {
                        let data = record.clone().data;
                        let json_data = to_json(&data).unwrap();
                        let rdp_entry = RDPEventEntry {
                            event_record_id: record.event_record_id,
                            event_id: event.system.event_id,
                            logon_type: test,
                            timestamp: record.timestamp,
                            description: "Remote interactive logon (RDP)".to_string(),
                            data: json_data,
                        };
                        rdp_usage_list.push(rdp_entry);
                    }
                }
            }
        }
        if event_id == 4778 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();

            let rdp_entry = RDPEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "Session Connected/Reconnected".to_string(),
                data: json_data,
            };
            rdp_usage_list.push(rdp_entry);
        }
        if event_id == 4779 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();

            let rdp_entry = RDPEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                logon_type: "".to_string(),
                timestamp: record.timestamp,
                description: "Session Disconnected".to_string(),
                data: json_data,
            };
            rdp_usage_list.push(rdp_entry);
        }
    }

    if rdp_usage_list.is_empty() {
        println!("Nothing to do :(");
        return Ok(());
    }

    write_json_lines(outfile, rdp_usage_list).expect("failed to write .json!");
    println!("Done! :)");
    Ok(())
}
