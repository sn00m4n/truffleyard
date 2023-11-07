use common::{find_software_hive, make_path};

use crate::account_usage::registry::user_accounts::get_profile_list;
use crate::errors::Error;

pub fn get_accountusage_registry_data(
    input: &String,
    outpath: &String,
    foldername: &String,
) -> Result<(), Error> {
    let output_path = make_path(outpath, foldername).unwrap();
    // SOFTWARE hive
    let software_hive = find_software_hive(input).unwrap();

    get_profile_list(&software_hive, output_path.clone())
        .expect("Failed to get Profile List Data!");

    Ok(())
}
