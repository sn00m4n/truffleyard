use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use common::VendorList;
use log::info;

use crate::parser::visit_dirs;

mod parser;

//Parser
#[derive(Parser)] //nimmt Quellcode von drunten, baut argparser (cli interface dings) optionen und so
struct Cli {
    ///input path for lists
    input_dir: String,
    ///output path
    output_path: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    env_logger::init();

    info!("Starting to read the dir");

    let mut vendors: VendorList = HashMap::new();
    visit_dirs(Path::new(&cli.input_dir), &mut vendors)?;

    let serde_vend = serde_json::to_vec_pretty(&vendors)?;

    let mut f = File::create(&cli.output_path)?;
    f.write_all(&serde_vend)?;

    Ok(())
}
