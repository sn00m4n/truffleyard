// Contains the code for the "Registry" Processing Mode
// Imports

use common::{find_software_hive, find_system_hive, make_path};

use crate::account_usage::registry::user_accounts::get_profile_list;
use crate::errors::Error;
use crate::external_device_usb_usage::registry::sof_volinfcache::sof_get_vic_data;
use crate::external_device_usb_usage::registry::sof_volname::sof_get_device_data;
use crate::external_device_usb_usage::registry::sys_hid::sys_get_hid_data;
use crate::external_device_usb_usage::registry::sys_mounteddev::sys_get_mounteddev_data;
use crate::external_device_usb_usage::registry::sys_scsi::sys_get_scsi_data;
use crate::external_device_usb_usage::registry::sys_usb::sys_get_usb_data;
use crate::external_device_usb_usage::registry::sys_usbstor::sys_get_usbstor_data;
use crate::system_information::registry::computer_name::get_computer_name;
use crate::system_information::registry::current_version::get_current_os_version;
use crate::system_information::registry::operating_system_version::get_os_updates;
use crate::system_information::registry::system_last_shutdown_time::get_shutdown_time;

pub fn get_registry_data(
    input: &String,
    outpath: &String,
    foldername: &String,
) -> Result<(), Error> {
    let output_path = make_path(outpath, foldername).unwrap();
    // SOFTWARE hive
    let software_hive = find_software_hive(input).unwrap();
    // SYSTEM hive
    let system_hive = find_system_hive(input).unwrap();

    // account usage
    get_profile_list(&software_hive, output_path.clone())
        .expect("Failed to get Profile List Data!");
    // usb devices
    sof_get_vic_data(&software_hive, output_path.clone())
        .expect("Failed to get Volume Info Cache!");
    //sof_get_device_data(&software_hive, blabla, output_path.clone()).expect("Failed to get Device Data!");
    //sys_get_hid_data(&system_hive, blabla, output_path.clone()).expect("Failed to get HID Data!");
    sys_get_mounteddev_data(&system_hive, output_path.clone())
        .expect("Failed to get Mounted Device Data!");
    sys_get_scsi_data(&system_hive, output_path.clone()).expect("Failed to get SCSI Data!");
    //sys_get_usb_data(&system_hive, blabla, output_path.clone()).expect("Failed to get USB Data!");
    sys_get_usbstor_data(&system_hive, output_path.clone()).expect("Failed ot get USBSTOR Data!");

    // system information
    get_computer_name(&system_hive, output_path.clone()).expect("Failed to get Computer Name!");
    get_current_os_version(&software_hive, output_path.clone())
        .expect("Failed to get Current OS Version!");
    get_os_updates(&software_hive, output_path.clone()).expect("Failed to get old OS Versions!");
    get_shutdown_time(&system_hive, output_path.clone())
        .expect("Failed to get last Shutdown Times!");
    Ok(())
}