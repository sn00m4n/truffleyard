// "Authentication Events identify where authentication of credentials occurred.
// They can be particularly useful when tracking local vs. domain account usage" - SANS Poster Windows Forensics, Authentication Events

use std::fs::File;
use std::io::{BufWriter, Write};

// Security.evtx
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use common::{parse_evtx, Event};
use serde::Serialize;
use serde_json::Value;
use xmltojson::to_json;
use {serde_xml_rs, xmltojson};

#[derive(Debug, Serialize)]
struct AuthenticationEventEntry {
    event_record_id: u64,
    event_id: u32,
    timestamp: DateTime<Utc>,
    description: String,
    data: Value,
}

pub fn sec_evtx_authentication_events_data(input: &str, outpath: &str) -> Result<()> {
    print!("Working on Authentication Events: ");
    let mut parser = parse_evtx(input).context("Failed to parse evtx!")?;
    let mut authentication_event_list: Vec<AuthenticationEventEntry> = Vec::new();
    for record in parser.records() {
        let record = record.context("Failed to get record!")?;
        let event: Event =
            serde_xml_rs::from_str(&record.data).context("Failed to create Event!")?;
        let event_id = event.system.event_id;

        // NTLM protocol: successful/failed account authentication
        if event_id == 4776 {
            let data = record.clone().data;
            let json_data = to_json(&data)?;

            let authentication_entry = AuthenticationEventEntry {
                event_record_id: record.event_record_id,
                event_id,
                timestamp: record.timestamp,
                description: "Successful/Failed account authentication".to_string(),
                data: json_data,
            };
            authentication_event_list.push(authentication_entry);
        }
        // Kerberos protocol: EventIDs 4768, 4769 and 4771
        // Ticket Granting Ticket was granted (successful logon)
        else if event_id == 4768 {
            let data = record.clone().data;
            let json_data = to_json(&data)?;
            let authentication_entry = AuthenticationEventEntry {
                event_record_id: record.event_record_id,
                event_id,
                timestamp: record.timestamp,
                description: "Ticket Granting Ticket was granted (successful logon)".to_string(),
                data: json_data,
            };
            authentication_event_list.push(authentication_entry);
        }
        // Service Ticket was requested (access to server resource)
        else if event_id == 4769 {
            let data = record.clone().data;
            let json_data = to_json(&data)?;
            let authentication_entry = AuthenticationEventEntry {
                event_record_id: record.event_record_id,
                event_id,
                timestamp: record.timestamp,
                description: "Service Ticket was requested (access to server resource)".to_string(),
                data: json_data,
            };
            authentication_event_list.push(authentication_entry);
        }
        // Pre-authentication failed (failed logon)
        else if event_id == 4771 {
            let data = record.clone().data;
            let json_data = to_json(&data)?;
            let authentication_entry = AuthenticationEventEntry {
                event_record_id: record.event_record_id,
                event_id,
                timestamp: record.timestamp,
                description: "Pre-authentication failed (failed logon))".to_string(),
                data: json_data,
            };
            authentication_event_list.push(authentication_entry);
        }
    }
    // check if list is empty so it doesnt create an empty file
    if authentication_event_list.is_empty() {
        println!("Nothing to do here, continuing with next job.");
        return Ok(());
    }

    let file = File::create(format!("{outpath}/evtx_authentication_events.json"))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &authentication_event_list)?;
    writer.flush()?;

    println!("Done here!");
    Ok(())
}
