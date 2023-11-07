// Contains the code for the "EventLogs" Processing Mode
// Imports
use common::{find_security_evtx, find_system_evtx, make_path};

// Account Usage
use crate::account_usage::eventlogs::authentication_events::sec_evtx_authentication_events_data;
use crate::account_usage::eventlogs::rdp_usage::sec_evtx_rdp_usage_data;
use crate::account_usage::eventlogs::service_events::{
    sec_evtx_service_events_data, sys_evtx_service_events_data,
};
use crate::account_usage::eventlogs::succ_faillogons::sys_evtx_logons_data;
use crate::errors::Error;

// function to get everything
pub fn get_eventlog_data(
    input: &String,
    outpath: &String,
    foldername: &String,
) -> Result<(), Error> {
    let output_path = make_path(outpath, foldername).unwrap();
    let system_evtx = find_system_evtx(input).unwrap();
    let security_evtx = find_security_evtx(input).unwrap();
    // account usage
    sec_evtx_authentication_events_data(&security_evtx, output_path.clone())
        .expect("Failed to get Authentication Event Data!");
    sec_evtx_rdp_usage_data(&security_evtx, output_path.clone())
        .expect("Failed to get RDP Usage Event Data!");
    sec_evtx_service_events_data(&security_evtx, output_path.clone())
        .expect("Failed to get Service Event Data from Security.evtx!");
    sys_evtx_service_events_data(&system_evtx, output_path.clone())
        .expect("Failed to get Service Event Data from System.evtx!");
    sys_evtx_logons_data(&system_evtx, output_path.clone())
        .expect("Failed to get Logon Event Data!");
    Ok(())
}
