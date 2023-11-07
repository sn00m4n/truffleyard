use common::find_software_hive;

use crate::account_usage::registry::user_accounts::get_profile_list;
use crate::errors::Error;

pub fn get_accountusage_registry_data(input: &str, outpath: &str) -> Result<(), Error> {
    // SOFTWARE hive
    let software_hive = find_software_hive(input).unwrap();

    get_profile_list(&software_hive, outpath).expect("Failed to get Profile List Data!");

    Ok(())
}
