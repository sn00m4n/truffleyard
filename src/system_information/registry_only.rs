// Contains the code for the "Registry" Processing Mode
// Imports

use anyhow::anyhow;
use common::{find_software_hive, find_system_hive};
use log::error;

use crate::system_information::registry::computer_name::get_computer_name;
use crate::system_information::registry::current_version::get_current_os_version;
use crate::system_information::registry::operating_system_version::get_os_updates;
use crate::system_information::registry::system_last_shutdown_time::get_shutdown_time;

pub fn get_systeminfo_registry_data(input: &str, outpath: &str) -> anyhow::Result<()> {
    let mut found_something = false;
    // SOFTWARE hive
    match find_software_hive(input) {
        Ok(path) => {
            found_something = true;
            if let Err(err) = get_current_os_version(&path, outpath) {
                error!("Failed to get Current OS Version: {err}")
            }
        }
        Err(err) => {
            error!("Could not find Software hive: {err}")
        }
    }
    // SYSTEM hive
    match find_system_hive(input) {
        Ok(path) => {
            found_something = true;
            if let Err(err) = get_computer_name(&path, outpath) {
                error!("Failed to get Computer Name: {err}")
            }

            if let Err(err) = get_os_updates(&path, outpath) {
                error!("Failed to get old OS Versions: {err}")
            }
            if let Err(err) = get_shutdown_time(&path, outpath) {
                error!("Failed to get last Shutdown Times: {err}")
            }
        }
        Err(err) => {
            error!("Could not find System hive: {err}")
        }
    }

    if !found_something {
        return Err(anyhow!("No .evtx found!"));
    }
    Ok(())
}
