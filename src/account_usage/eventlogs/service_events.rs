// "analyze logs for suspicious windows service creation, persistence, and services started or
// stopped around the time of a suspected compromise. service events also record account information" - SANS Poster Windows Forensics

use std::fs::File;
use std::io::{BufWriter, Write};

use chrono::{DateTime, Utc};
use common::{parse_evtx, Event};
use serde::Serialize;
use serde_json::Value;
use xmltojson::to_json;
use {serde_xml_rs, xmltojson};

use crate::errors::Error;

#[derive(Debug, Serialize)]
struct ServiceEventEntry {
    event_record_id: u64,
    event_id: u32,
    timestamp: DateTime<Utc>,
    description: String,
    data: Value,
}

// TO-DO!!: clean up code, seperate system & security evtx more clearly!
// Windows 7+
pub fn sys_evtx_service_events_data(input: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on Service Events (System.evtx): ");
    let mut parser = parse_evtx(input).unwrap();
    let mut service_event_list: Vec<ServiceEventEntry> = Vec::new();
    for record in parser.records() {
        let record = record.unwrap();
        let event: Event = serde_xml_rs::from_str(&record.data).unwrap();

        // event id 7034 -> service crashed unexpectedly
        if event.system.event_id == 7034 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();

            let service_entry = ServiceEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                timestamp: record.timestamp,
                description: "A service crashed unexpectedly".to_string(),
                data: json_data,
            };
            service_event_list.push(service_entry);
        }
        // event id 7035 -> service sent start/stop control
        else if event.system.event_id == 7035 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let service_entry = ServiceEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                timestamp: record.timestamp,
                description: "A service sent a Start/Stop control".to_string(),
                data: json_data,
            };
            service_event_list.push(service_entry);
        }

        // event id 7036 -> service started or stopped
        if event.system.event_id == 7036 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let service_entry = ServiceEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                timestamp: record.timestamp,
                description: "A service started or stopped".to_string(),
                data: json_data,
            };
            service_event_list.push(service_entry);
        }

        // event id 7040 -> start type changed
        if event.system.event_id == 7040 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let service_entry = ServiceEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                timestamp: record.timestamp,
                description: "The start type changed (Boot|On request|Disabled)".to_string(),
                data: json_data,
            };
            service_event_list.push(service_entry);
        }

        // event id 7045 -> service was installed on system
        if event.system.event_id == 7045 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();
            let service_entry = ServiceEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                timestamp: record.timestamp,
                description: "A service was installed on the system".to_string(),
                data: json_data,
            };
            service_event_list.push(service_entry);
        }
    }

    // check if list empty
    if service_event_list.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }

    let file = File::create(format!("{outpath}/evtx_sys_service_events_usage.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &service_event_list)?;
    writer.flush()?;
    println!("Done here!");
    Ok(())
}

// Windows 10+
pub fn sec_evtx_service_events_data(input: &str, outpath: &str) -> Result<(), Error> {
    print!("Working on Service Events (Security.evtx): ");
    let mut parser = parse_evtx(input).unwrap();
    let mut service_event_list: Vec<ServiceEventEntry> = Vec::new();
    for record in parser.records() {
        let record = record.unwrap();
        let event: Event = serde_xml_rs::from_str(&record.data).unwrap();
        if event.system.event_id == 4697 {
            let data = record.clone().data;
            let json_data = to_json(&data).unwrap();

            //WINDOWS 10+ only!:
            let service_entry = ServiceEventEntry {
                event_record_id: record.event_record_id,
                event_id: event.system.event_id,
                timestamp: record.timestamp,
                description: "A service was installed on the system".to_string(),
                data: json_data,
            };
            service_event_list.push(service_entry);
        }
    }
    if service_event_list.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }

    let file = File::create(format!("{outpath}/evtx_sec_service_events_usage.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &service_event_list)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
