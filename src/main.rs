mod account_usage;
mod application_execution;
mod browser_activity;
mod cloud_storage;
mod deleted_items_file_existence;
pub mod errors;
mod external_device_usb_usage;
mod file_folder_opening;
mod network_activity_physical_location;
mod system_information;
//mod eventlogs;
//utf16le

use std::env;

use clap::{Parser, Subcommand};
use common::{find_security_evtx, find_software_hive, find_system_evtx, find_system_hive};

use crate::account_usage::registry::user_accounts::get_profile_list;

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
    let cli = Cli::parse();
    let sof_file = find_software_hive(&cli.filepath_image).unwrap_or("file not found".to_string());

    get_profile_list(&sof_file, "testfiles/profile_list.json".to_string())
        .expect("if you see this, i messed up");
}
