pub mod errors;
mod eventlogs;
mod registry;
//mod eventlogs;
//utf16le

use std::env;

use clap::{Parser, Subcommand};
use common::{find_security_evtx, find_software_hive, find_system_evtx, find_system_hive};

use crate::eventlogs::account_usage::rdp_usage::evtx_rdp_usage_data;
use crate::eventlogs::account_usage::service_events::{
    sec_evtx_service_events_data, sys_evtx_service_events_data,
};
use crate::registry::application_execution::shimcache::shimcache_data;
use crate::registry::external_devices::sof_volname::sof_get_device_data;
use crate::registry::external_devices::sys_hid::sys_get_hid_data;
use crate::registry::external_devices::sys_mounteddev::sys_get_mounteddev_data;
use crate::registry::external_devices::sys_scsi::sys_get_scsi_data;
use crate::registry::external_devices::sys_usb::sys_get_usb_data;
use crate::registry::external_devices::sys_usbstor::sys_get_usbstor_data;
use crate::registry::system_information::computer_name::get_computer_name;
use crate::registry::system_information::current_version::get_current_os_version;
use crate::registry::system_information::operating_system_version::get_os_updates;
use crate::registry::system_information::system_last_shutdown_time::get_shutdown_time;

/*#[derive(Subcommand)]
enum Sub {
    /// get registry data only
    Registry, /*{
                   #[arg(default_value_t = String::from("testfiles/"))]
                   output_filepath: String,
              },*/
}*/

#[derive(Parser)]
#[command(
    author = "ally",
    version = "0.1",
    about = "Soon to be nice tool for forensic stuff :)"
)]
struct Cli {
    ///path where mounted image is located
    #[arg(short)]
    filepath_image: String,
    /*#[clap(subcommand)]
    subcommand: Sub,*/
}

fn main() {
    /* if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "truffleyard=TRACE");
    }
    env_logger::init();*/

    // let vidpidjson = "./out.json".to_string();
    let cli = Cli::parse();
    let sec_file = find_security_evtx(&cli.filepath_image).unwrap_or("file not found".to_string());

    evtx_rdp_usage_data(&sec_file, "testfiles/rdp_data_test.json".to_string())
        .expect("if you see this, i fucked up");

    /*match cli.subcommand {
        Sub::Registry => {
            //SYSTEM HIVE
            let sys_file =
                find_system_hive(&cli.filepath_image).unwrap_or("file not found".to_string());
            sys_get_usbstor_data(&sys_file, "testfiles/sys_usbstortest.json".to_string());
            sys_get_usb_data(
                &sys_file,
                &vidpidjson,
                "testfiles/sys_usbtest.json".to_string(),
            );
            sys_get_hid_data(
                &sys_file,
                &vidpidjson,
                "testfiles/sys_hidtest.json".to_string(),
            );
            sys_get_scsi_data(&sys_file, "testfiles/sys_scsitest.json".to_string());

            //SOFTWARE HIVE
            let sof_file =
                find_software_hive(&cli.filepath_image).unwrap_or("file not found".to_string());
            sof_get_device_data(
                &sof_file,
                &vidpidjson,
                "testfiles/sof_devicedata.json".to_string(),
            );
            sof_get_vic_data(&sof_file, "testfiles/sof_vicdata.json".to_string());
        }
    }*/
}
