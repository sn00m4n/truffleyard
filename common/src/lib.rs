use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::num::ParseIntError;

use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};
use encoding_rs::UTF_16LE;
use evtx::{EvtxParser, ParserSettings};
use serde::{Deserialize, Serialize};

// Types:
pub type VendorList = HashMap<u16, Vendor>;

// Structs:
///Struct for saving the vendors and their products for comparison with vid, pid
#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub name: Option<String>,
    pub vid: u16,
    pub devices: HashMap<u16, Device>,
}

impl Vendor {
    pub fn new(vid: u16, name: Option<String>) -> Vendor {
        Vendor {
            name,
            vid,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.did, device);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub did: u16,
    pub name: Option<String>,
}

// structs for eventlog stuff
/// Struct for whole event
#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename = "System")]
    pub system: System,
    #[serde(rename = "EventData")]
    pub event_data: Option<EventData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EventData {
    #[serde(rename = "$value", default)]
    pub events: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Data {
    #[serde(rename = "Name")]
    pub name: OuterName,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum OuterName {
    Known(Name),
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Name {
    SubjectUserSid,
    SubjectUserName,
    SubjectDomainName,
    SubjectLogonId,
    TargetUserSid,
    TargetUserName,
    TargetDomainName,
    TargetLogonId,
    LogonType,
    LogonProcessName,
    AuthenticationPackageName,
    WorkstationName,
    LogonGuid,
    TransmittedServices,
    LmPackageName,
    KeyLength,
    ProcessId,
    ProcessName,
    IpAddress,
    IpPort,
    ImpersonationLevel,
    RestrictedAdminMode,
    TargetOutboundUserName,
    TargetOutboundDomainName,
    VirtualAccount,
    TargetLinkedLogonId,
    ElevatedToken,
    MandatoryLabel,
    NewProcessId,
    NewProcessName,
    TokenElevationType,
    CommandLine,
    ParentProcessName,
    TargetLogonGuid,
    TargetServerName,
    TargetInfo,
    PreviousTime,
    NewTime,
    TargetProcessId,
    TargetProcessName,
    LoadOptions,
    DisableIntegrityChecks,
    HypervisorDebug,
    Status,
    PackageName,
    RemoteEventLogging,
    VsmLaunchType,
    HypervisorLaunchType,
    TestSigning,
    AdvancedOptions,
    SubStatus,
    KernelDebug,
    Workstation,
    FlightSigning,
    FailureReason,
    ConfigAccessPolicy,
    HypervisorLoadOptions,
    PuaCount,
    TargetSid,
    PuaPolicyId,
    AccessGranted,
    PrivilegeList,
    SamAccountName,
    SidHistory,
    MemberName,
    Dummy,
    DisplayName,
    AccessRemoved,
    MemberSid,
    UserPrincipalName,
    CallerProcessId,
}
#[derive(Deserialize, Debug, Clone)]
pub struct System {
    #[serde(rename = "EventID")]
    pub event_id: u32,
}

// different useful functions:
// conversions:
///converts hex to int
pub fn convert_to_int(hex: &str) -> Result<u16, ParseIntError> {
    let no_pref = hex.trim_start_matches("0x");
    u16::from_str_radix(no_pref, 16)
}

///converts int to hex
pub fn convert_to_hex(int: u16) -> String {
    format!("{:#06X}", int)
}

///converts wintime to UTC
pub fn convert_win_time(wintime: u64) -> DateTime<Utc> {
    let date: DateTime<Utc> = DateTime::from_utc(
        NaiveDate::from_ymd_opt(1601, 1, 1)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        Utc,
    );

    date + Duration::microseconds((wintime / 10) as i64)
}

// find needed files
///takes path of mounted filesystem and finds the system hive file (SYSTEM)
pub fn find_system_hive(mnt_image_path: &String) -> Result<String, Error> {
    Ok(mnt_image_path.to_owned() + "/Windows/System32/config/SYSTEM")
    //File::open(system_hive_path)
}
///takes path of mounted filesystem and finds the software hive file (SOFTWARE)
pub fn find_software_hive(mnt_image_path: &String) -> Result<String, Error> {
    Ok(mnt_image_path.to_owned() + "/Windows/System32/config/SOFTWARE")
}
///takes path of mounted filesystem and finds the system evtx file (System.evtx)
pub fn find_system_evtx(mnt_image_path: &String) -> Result<String, Error> {
    Ok(mnt_image_path.to_owned() + "/Windows/System32/winevt/Logs/System.evtx")
}
///takes path of mounted filesystem and finds the security evtx file (Security.evtx)
pub fn find_security_evtx(mnt_image_path: &String) -> Result<String, Error> {
    Ok(mnt_image_path.to_owned() + "/Windows/System32/winevt/Logs/Security.evtx")
}

pub fn parse_evtx(input: &String) -> Result<EvtxParser<File>, Error> {
    let settings = ParserSettings::default().separate_json_attributes(true);
    let parser = EvtxParser::from_path(input)
        .unwrap()
        .with_configuration(settings);
    Ok(parser)
}

/// converts to human readable ascii ('cuz windows uses encodings however it feels like on that day ig)
pub fn read_extended_ascii(buf: &[u8], offset: usize, length: usize) -> Option<String> {
    if buf.len() - offset < length {
        return None;
    }

    let raw: Vec<u8> = buf[offset..(offset + length)]
        .iter()
        .copied()
        .flat_map(|x| [x, 0u8])
        .collect();
    Some(UTF_16LE.decode(&raw).0.to_string())
}
