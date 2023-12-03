#![warn(clippy::unwrap_used, clippy::expect_used)]

mod account_usage;
//mod application_execution;
//mod browser_activity;
//mod cloud_storage;
//mod deleted_items_file_existence;
mod eventlogs;
mod external_device_usb_usage;
//mod file_folder_opening;
//mod network_activity_physical_location;
mod registry;
mod system_information;
//mod tests;

//use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use common::make_path;
use log::error;

use crate::account_usage::eventlogs_only::get_accountusage_eventlog_data;
use crate::account_usage::registry_only::get_accountusage_registry_data;
use crate::eventlogs::get_eventlog_data;
use crate::external_device_usb_usage::registry_only::get_externaldevice_registry_data;
use crate::registry::get_registry_data;
use crate::system_information::registry_only::get_systeminfo_registry_data;
//use crate::tests::test::testing;

#[derive(Parser)]
#[command(
    author = "ally",
    version = "0.1",
    about = "
 _____            __  __ _                          _ 
|_   _| __ _   _ / _|/ _| | ___ _   _  __ _ _ __ __| |
  | || '__| | | | |_| |_| |/ _ \\ | | |/ _` | '__/ _` |
  | || |  | |_| |  _|  _| |  __/ |_| | (_| | | | (_| |
  |_||_|   \\__,_|_| |_| |_|\\___|\\__, |\\__,_|_|  \\__,_|
                                |___/                 
                                "
)]
struct Cli {
    /// path where mounted image is located
    #[arg(short)]
    image_path: String,
    /// output path, default is working directory
    #[arg(short, default_value = ".")]
    output_path: String,
    /// name of result-folder, default is "results"
    #[arg(short, long, default_value = "results")]
    folder_name: String,
    /// path to file that contains vid&pid  
    #[arg(short)]
    vidpid_path: String,
    /// specifying Subcommands
    #[clap(subcommand)]
    command: Commands,
}

/// The different Processing Modes
#[derive(ValueEnum, Copy, Clone)]
enum ProcessingMode {
    RegistryOnly,
    EventLogOnly,
    All,
}
// implementing Display for Processing Modes, so it shows up in CLI
impl Display for ProcessingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingMode::RegistryOnly => write!(f, "registry-only"),
            ProcessingMode::EventLogOnly => write!(f, "eventlog-only"),
            ProcessingMode::All => write!(f, "all"),
        }
    }
}

/// The different subcommands implemented so far
#[derive(Subcommand)]
enum Commands {
    /// Analyzes everything (that's implemented so far)
    All,
    /// Analyzes only Registry artifacts (that are implemented so far)
    Registry,
    /// Analyzes only EventLog artifacts (that are implemented so far)
    EventLogs,
    /// Analyzes Account Usage artifacts
    AccountUsage {
        #[arg(short, default_value_t = ProcessingMode::All)]
        mode: ProcessingMode,
    },
    /// Analyzes External Devices and USB usage artifacts
    ExternalDevices {
        #[arg(short, default_value_t = ProcessingMode::All)]
        mode: ProcessingMode,
    },
    /// Analyzes System Information artifacts
    SystemInformation {
        #[arg(short, default_value_t = ProcessingMode::All)]
        mode: ProcessingMode,
    },
    // to be implemented:
    /* ApplicationExecution, BrowserActivity, CloudStorage, DeletedItems, FileFolderOpening, NetworkActivity*/
}

fn main() -> Result<()> {
    let before = Instant::now();
    let cli = Cli::parse();
    match cli.command {
        Commands::All => {
            let path = format!("{}/{}", cli.output_path, cli.folder_name);
            let out_put_path = make_path(path).context("Failed to create directory!")?;
            let before2 = Instant::now();
            if let Err(err) = get_eventlog_data(&cli.image_path, &out_put_path) {
                error!("Failed to get EventLog Data: {err}")
            }
            if let Err(err) = get_registry_data(&cli.image_path, &out_put_path, &cli.vidpid_path) {
                error!("Failed to get Registry Data: {err}")
            }
            println!("All done!");
            println!(
                "Elapsed time from very beginning (hitting enter): {:.2?}",
                before.elapsed()
            );
            println!(
                "Elapsed time from actual start of first extracting command:: {:.2?}",
                before2.elapsed()
            );
            Ok(())
        }
        Commands::Registry => {
            let path = format!("{}/{}", cli.output_path, cli.folder_name);
            let out_put_path = make_path(path).context("Failed to create directory!")?;
            let before2 = Instant::now();
            if let Err(err) = get_registry_data(&cli.image_path, &out_put_path, &cli.vidpid_path) {
                error!("Failed to get Registry Data: {err}")
            }
            println!("All done!");
            println!(
                "Elapsed time from very beginning (hitting enter): {:.2?}",
                before.elapsed()
            );
            println!(
                "Elapsed time from actual start of first extracting command:: {:.2?}",
                before2.elapsed()
            );
            Ok(())
        }
        Commands::EventLogs => {
            let path = format!("{}/{}", cli.output_path, cli.folder_name);
            let out_put_path = make_path(path).context("Failed to create directory!")?;
            let before2 = Instant::now();
            if let Err(err) = get_eventlog_data(&cli.image_path, &out_put_path) {
                error!("Failed to get EventLog Data: {err}")
            }
            println!("All done!");
            println!(
                "Elapsed time from very beginning (hitting enter): {:.2?}",
                before.elapsed()
            );
            println!(
                "Elapsed time from actual start of first extracting command: {:.2?}",
                before2.elapsed()
            );
            Ok(())
        }
        Commands::AccountUsage { mode } => match mode {
            ProcessingMode::RegistryOnly => {
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory!")?;
                let before2 = Instant::now();
                if let Err(err) = get_accountusage_registry_data(&cli.image_path, &out_put_path) {
                    error!("Failed to get Registry Data for Account Usage: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
            ProcessingMode::EventLogOnly => {
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory!")?;
                let before2 = Instant::now();
                if let Err(err) = get_accountusage_eventlog_data(&cli.image_path, &out_put_path) {
                    error!("Failed to get EventLog Data for Account Usage: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
            ProcessingMode::All => {
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory!")?;
                let before2 = Instant::now();
                if let Err(err) = get_accountusage_eventlog_data(&cli.image_path, &out_put_path) {
                    error!("Failed to get EventLog Data for Account Usage: {err}")
                }
                if let Err(err) = get_accountusage_registry_data(&cli.image_path, &out_put_path) {
                    error!("Failed to get Registry Data for Account Usage: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
        },
        Commands::ExternalDevices { mode } => match mode {
            ProcessingMode::RegistryOnly => {
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory!")?;
                let before2 = Instant::now();
                if let Err(err) = get_externaldevice_registry_data(
                    &cli.image_path,
                    &out_put_path,
                    &cli.vidpid_path,
                ) {
                    error!("Failed to get Registry Data for External Devices: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
            ProcessingMode::EventLogOnly => {
                println!("Sorry, not implemented yet!");
                Ok(())
            }
            ProcessingMode::All => {
                println!("EventLogs are not implemented yet, will continue with Registry Only!");
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory: {err}")?;
                let before2 = Instant::now();
                if let Err(err) = get_externaldevice_registry_data(
                    &cli.image_path,
                    &out_put_path,
                    &cli.vidpid_path,
                ) {
                    error!("Failed to get Registry Data for External Devices: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
        },
        Commands::SystemInformation { mode } => match mode {
            ProcessingMode::RegistryOnly => {
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory!")?;
                let before2 = Instant::now();
                if let Err(err) = get_systeminfo_registry_data(&cli.image_path, &out_put_path) {
                    error!("Failed to get Registry Data for System Information: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
            ProcessingMode::EventLogOnly => {
                println!("Sorry, not implemented yet!");
                Ok(())
            }
            ProcessingMode::All => {
                println!("EventLogs are not implemented yet, will continue with Registry Only!");
                let path = format!("{}/{}", cli.output_path, cli.folder_name);
                let out_put_path = make_path(path).context("Failed to create directory!")?;
                let before2 = Instant::now();
                if let Err(err) = get_systeminfo_registry_data(&cli.image_path, &out_put_path) {
                    error!("Failed to get Registry Data for System Information: {err}")
                }
                println!("All done!");
                println!(
                    "Elapsed time from very beginning (hitting enter): {:.2?}",
                    before.elapsed()
                );
                println!(
                    "Elapsed time from actual start of first extracting command: {:.2?}",
                    before2.elapsed()
                );
                Ok(())
            }
        },
    }
}
