use anyhow::anyhow;
use common::find_software_hive;
use log::error;

use crate::account_usage::registry::user_accounts::get_profile_list;

pub fn get_accountusage_registry_data(input: &str, outpath: &str) -> anyhow::Result<()> {
    // SOFTWARE hive
    let mut found_something = false;

    match find_software_hive(input) {
        Ok(path) => {
            found_something = true;
            if let Err(err) = get_profile_list(&path, outpath) {
                error!("Failed to get Profile List Data: {err}")
            }
        }
        Err(err) => {
            error!("Could not find Software hive: {err}")
        }
    }

    if !found_something {
        return Err(anyhow!("No Software hive found!"));
    }

    Ok(())
}
