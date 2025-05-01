use std::{error::Error, ffi::OsStr, path::PathBuf};

use types::PluginInfo;
use vst2::scan_vst2;
use vst3::scan_vst3;

pub mod scan;
pub mod types;
pub mod utils;
pub mod vst2;
pub mod vst3;

pub fn scan_file(path: &PathBuf) -> Result<PluginInfo, Box<dyn Error>> {
    if let Some(ext) = path.extension() {
        if ext == OsStr::new("vst3") {
            return scan_vst3(&path).map(PluginInfo::Vst3);
        }

        if ext == OsStr::new("dll") {
            return scan_vst2(&path).map(PluginInfo::Vst2);
        }
    }

    Err("The file extension isn't correct. Expected to be one of: 'vst3', 'dll'".into())
}

pub fn scan_directory(path: &PathBuf) -> Result<Vec<PluginInfo>, Box<dyn Error>> {
    if !path.is_dir() {
        return Err(format!("{} is not a directory", path.display()).into());
    }

    let mut collected = vec![];

    for entry in path.read_dir()? {
        match entry {
            Err(err) => println!("Error reading entry in dir {}: {}", path.display(), err),
            Ok(entry) => {
                let nested_path = entry.path();
                if nested_path.is_dir() {
                    match scan_directory(&nested_path) {
                        Err(err) => println!("Error reading {}: {}", nested_path.display(), err),
                        Ok(mut data) => collected.append(&mut data),
                    }
                } else {
                    if let Some(file_ext) = nested_path.extension() {
                        if file_ext == OsStr::new("dll") || file_ext == OsStr::new("vst3") {
                            match scan_file(&nested_path) {
                                Err(err) => println!("Error scanning {}: {}", nested_path.display(), err),
                                Ok(info) => collected.push(info),
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(collected)
}
