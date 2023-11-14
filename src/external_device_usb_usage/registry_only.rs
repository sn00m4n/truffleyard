use anyhow::anyhow;
// RegistryOnly Processing Mode for External Devices
use common::{find_software_hive, find_system_hive};
use log::error;

use crate::external_device_usb_usage::registry::sof_volinfcache::sof_get_vic_data;
use crate::external_device_usb_usage::registry::sof_volname::sof_get_device_data;
use crate::external_device_usb_usage::registry::sys_hid::sys_get_hid_data;
use crate::external_device_usb_usage::registry::sys_mounteddev::sys_get_mounteddev_data;
use crate::external_device_usb_usage::registry::sys_scsi::sys_get_scsi_data;
use crate::external_device_usb_usage::registry::sys_usb::sys_get_usb_data;
use crate::external_device_usb_usage::registry::sys_usbstor::sys_get_usbstor_data;

pub fn get_externaldevice_registry_data(input: &str, outpath: &str) -> anyhow::Result<()> {
    let mut found_something = false;
    let vidpidjson = "testlists/output.json";
    // SOFTWARE hive
    match find_software_hive(input) {
        Ok(path) => {
            found_something = true;
            // usb devices
            if let Err(err) = sof_get_vic_data(&path, outpath) {
                error!("Failed to get Volume Info Cache: {err}")
            }
            if let Err(err) = sof_get_device_data(&path, vidpidjson, outpath) {
                error!("Failed to get Device Data: {err}")
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
            if let Err(err) = sys_get_hid_data(&path, vidpidjson, outpath) {
                error!("Failed to get HID Data: {err}")
            }
            if let Err(err) = sys_get_mounteddev_data(&path, outpath) {
                error!("Failed to get Mounted Device Data: {err}")
            }
            if let Err(err) = sys_get_scsi_data(&path, outpath) {
                error!("Failed to get SCSI Data: {err}")
            }
            if let Err(err) = sys_get_usb_data(&path, vidpidjson, outpath) {
                error!("Failed to get USB Data: {err}")
            }
            if let Err(err) = sys_get_usbstor_data(&path, outpath) {
                error!("Failed ot get USBSTOR Data: {err}")
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
