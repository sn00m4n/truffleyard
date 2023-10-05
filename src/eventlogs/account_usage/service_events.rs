// "analyze logs for suspicious windows service creation, persistence, and services started or
// stopped around the time of a suspected compromise. service events also record account information" - SANS Poster Windows Forensics

//TO-DO

use std::path::Path;

use chrono::{DateTime, Utc};
use evtx::{EvtxParser, ParserSettings};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::eventlogs::errors::Error;
use crate::eventlogs::errors::Error::EvtxError;

#[derive(Debug, Serialize)]
pub struct Record {
    pub event_record_id: u64,
    pub timestamp: DateTime<Utc>,
    pub data: Box<RawValue>,
}

#[derive(Debug, Serialize)]
struct ServiceEventEntry {
    event_id: u32,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Event {
    #[serde(rename = "System")]
    system: System,
}

#[derive(Deserialize, Debug)]
struct System {
    #[serde(rename = "EventID")]
    event_id: u32,
}

pub fn evtx_service_events_data(input: &String) {
    //let mut jsonlist = Vec::new();
    let mut parser = EvtxParser::from_path(&input).unwrap();
    for record in parser.records() {
        let record = record.unwrap();
        let event: Event = serde_xml_rs::from_str(&record.data).unwrap();

        if event.system.event_id == 7034 {

            /*if event.system.event_id == 7035 {}
            if event.system.event_id == 7036 {}
            if event.system.event_id == 7040 {}
            if event.system.event_id == 7045 {}*/
        }
    }
}
