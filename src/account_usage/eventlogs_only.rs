use common::{find_security_evtx, find_system_evtx};

use crate::account_usage::eventlogs::authentication_events::sec_evtx_authentication_events_data;
use crate::account_usage::eventlogs::rdp_usage::sec_evtx_rdp_usage_data;
use crate::account_usage::eventlogs::service_events::{
    sec_evtx_service_events_data, sys_evtx_service_events_data,
};
use crate::account_usage::eventlogs::succ_faillogons::sec_evtx_logons_data;
use crate::errors::Error;

pub fn get_accountusage_eventlog_data(input: &str, outpath: &str) -> Result<(), Error> {
    let system_evtx = find_system_evtx(input).unwrap();
    let security_evtx = find_security_evtx(input).unwrap();

    sec_evtx_authentication_events_data(&security_evtx, outpath)
        .expect("Failed to get Authentication Event Data!");
    sec_evtx_rdp_usage_data(&security_evtx, outpath).expect("Failed to get RDP Usage Event Data!");
    sec_evtx_service_events_data(&security_evtx, outpath)
        .expect("Failed to get Service Event Data from Security.evtx!");
    sys_evtx_service_events_data(&system_evtx, outpath)
        .expect("Failed to get Service Event Data from System.evtx!");
    sec_evtx_logons_data(&security_evtx, outpath).expect("Failed to get Logon Event Data!");
    Ok(())
}
