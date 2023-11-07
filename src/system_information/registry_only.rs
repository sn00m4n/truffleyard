// Contains the code for the "Registry" Processing Mode
// Imports

use common::{find_software_hive, find_system_hive, make_path};

use crate::errors::Error;
use crate::system_information::registry::computer_name::get_computer_name;
use crate::system_information::registry::current_version::get_current_os_version;
use crate::system_information::registry::operating_system_version::get_os_updates;
use crate::system_information::registry::system_last_shutdown_time::get_shutdown_time;

pub fn get_systeminfo_registry_data(
    input: &String,
    outpath: &String,
    foldername: &String,
) -> Result<(), Error> {
    let output_path = make_path(outpath, foldername).unwrap();
    // SOFTWARE hive
    let software_hive = find_software_hive(input).unwrap();
    // SYSTEM hive
    let system_hive = find_system_hive(input).unwrap();

    // system information
    get_computer_name(&system_hive, output_path.clone()).expect("Failed to get Computer Name!");
    get_current_os_version(&software_hive, output_path.clone())
        .expect("Failed to get Current OS Version!");
    get_os_updates(&software_hive, output_path.clone()).expect("Failed to get old OS Versions!");
    get_shutdown_time(&system_hive, output_path.clone())
        .expect("Failed to get last Shutdown Times!");
    Ok(())
}
