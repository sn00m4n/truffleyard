// RegistryOnly Processing Mode for External Devices
use common::{find_software_hive, find_system_hive};

use crate::errors::Error;
use crate::external_device_usb_usage::registry::sof_volinfcache::sof_get_vic_data;
use crate::external_device_usb_usage::registry::sys_mounteddev::sys_get_mounteddev_data;
use crate::external_device_usb_usage::registry::sys_scsi::sys_get_scsi_data;
use crate::external_device_usb_usage::registry::sys_usbstor::sys_get_usbstor_data;

pub fn get_externaldevice_registry_data(input: &str, outpath: &str) -> Result<(), Error> {
    // SOFTWARE hive
    let software_hive = find_software_hive(input).unwrap();
    // SYSTEM hive
    let system_hive = find_system_hive(input).unwrap();

    // usb devices
    sof_get_vic_data(&software_hive, outpath).expect("Failed to get Volume Info Cache!");
    //sof_get_device_data(&software_hive, blabla, outpath).expect("Failed to get Device Data!");
    //sys_get_hid_data(&system_hive, blabla, outpath).expect("Failed to get HID Data!");
    sys_get_mounteddev_data(&system_hive, outpath).expect("Failed to get Mounted Device Data!");
    sys_get_scsi_data(&system_hive, outpath).expect("Failed to get SCSI Data!");
    //sys_get_usb_data(&system_hive, blabla, outpath).expect("Failed to get USB Data!");
    sys_get_usbstor_data(&system_hive, outpath).expect("Failed ot get USBSTOR Data!");

    Ok(())
}
