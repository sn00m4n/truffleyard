use std::fs;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::anyhow;
use common::{convert_to_int, Device, Vendor};
use log::error;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::VendorList;

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new("(idVendor\\s+(?<vid>\\w+)\\s(?<vname>(?:\\S+\\s)*)\\s+(idProduct\\s+(?<pid>\\w+)\\s(?<pname>(?:\\S+\\s)*)))+").unwrap()
});

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, vendors: &mut VendorList) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry.map_err(|e| anyhow!("DirEntry invalid: {e}"))?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, vendors)?;
            } else if path.is_file() && path.file_name().unwrap() != "README.md" {
                parser(&path, vendors)?;
            }
        }
    }
    Ok(())
}

fn parser(path: &Path, vendors: &mut VendorList) -> anyhow::Result<()> {
    let lines = match read_to_string(path) {
        Ok(v) => v,
        Err(err) => {
            error!(
                "Could not read file: {}, Reason: {err}",
                path.to_str().unwrap()
            );
            return Ok(());
        }
    };

    let captures = RE.captures_iter(&lines);
    for c in captures {
        let Some(pid_str) = c.name("pid") else {
            return Err(anyhow!("pid match was not found"));
        };

        let pid_int =
            convert_to_int(pid_str.as_str()).map_err(|e| anyhow!("Conversion2Int failed: {e}"))?;
        let device = if let Some(p) = c.name("pname") {
            Device {
                did: pid_int,
                name: Some(p.as_str().trim().to_string()),
            }
        } else {
            Device {
                did: pid_int,
                name: None,
            }
        };
        let Some(vid_str) = c.name("vid") else {
            return Err(anyhow!("vid match was not found"));
        };
        let vid_int =
            convert_to_int(vid_str.as_str()).map_err(|e| anyhow!("Conversion2Int failed: {e}"))?;

        if vendors.contains_key(&vid_int) {
            vendors.get_mut(&vid_int).unwrap().add_device(device);
        } else {
            let mut vendor = if let Some(v) = c.name("vname") {
                Vendor::new(vid_int, Some(v.as_str().trim().to_string()))
            } else {
                Vendor::new(vid_int, None)
            };
            vendor.add_device(device);
            vendors.insert(vendor.vid, vendor);
        }
    }

    Ok(())
}
