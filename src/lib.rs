use std::{error::Error, ffi::OsStr, path::Path};

use types::PluginInfo;
use vst2::scan_vst2;
use vst3::scan_vst3;

pub mod arch;
pub mod lib_loader;
pub mod scan;
pub mod types;
pub mod utils;
pub mod vst2;
pub mod vst3;

pub fn scan_file(path: &Path) -> Result<PluginInfo, Box<dyn Error>> {
    if let Some(ext) = path.extension() {
        if ext == OsStr::new("vst3") {
            let loader = scan_vst3(path)?;
            let vst3_info = loader.read_info()?;
            return Ok(PluginInfo::Vst3(vst3_info));
        }

        if ext == OsStr::new("dll") {
            return scan_vst2(path).map(PluginInfo::Vst2);
        }
    }

    Err("The file extension isn't correct. Expected to be one of: 'vst3', 'dll'".into())
}
