use std::{error::Error, ffi::OsStr, path::PathBuf};

use types::PluginInfo;
use vst2::scan_vst2;
use vst3::scan_vst3;

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
