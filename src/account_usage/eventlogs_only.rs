use anyhow::anyhow;
use common::{find_security_evtx, find_system_evtx};
use log::error;

use crate::account_usage::eventlogs::authentication_events::sec_evtx_authentication_events_data;
use crate::account_usage::eventlogs::rdp_usage::sec_evtx_rdp_usage_data;
use crate::account_usage::eventlogs::service_events::{
    sec_evtx_service_events_data, sys_evtx_service_events_data,
};
use crate::account_usage::eventlogs::succ_faillogons::sec_evtx_logons_data;

pub fn get_accountusage_eventlog_data(input: &str, outpath: &str) -> anyhow::Result<()> {
    let mut found_something = false;

    match find_system_evtx(input) {
        Ok(path) => {
            found_something = true;
            if let Err(err) = sys_evtx_service_events_data(&path, outpath) {
                error!("Failed to get Service Event Data from System.evtx: {err}")
            }
        }
        Err(err) => {
            error!("Could not find System.evtx: {err}");
        }
    }

    match find_security_evtx(input) {
        Ok(path) => {
            found_something = true;
            if let Err(err) = sec_evtx_rdp_usage_data(&path, outpath) {
                error!("Failed to get RDP Usage Event Data: {err}")
            }
            if let Err(err) = sec_evtx_service_events_data(&path, outpath) {
                error!("Failed to get Service Event Data from Security.evtx: {err}")
            }

            if let Err(err) = sec_evtx_logons_data(&path, outpath) {
                error!("Failed to get Logon Event Data: {err}")
            }

            if let Err(err) = sec_evtx_authentication_events_data(&path, outpath) {
                error!("Failed to get Authentication Event Data: {err}");
            }
        }
        Err(err) => {
            error!("Could not find Security.evtx: {err}");
        }
    }

    if !found_something {
        return Err(anyhow!("No .evtx found!"));
    }

    Ok(())
}
