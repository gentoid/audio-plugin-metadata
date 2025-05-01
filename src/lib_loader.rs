use libloading::Library;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;

use windows_sys::Win32::Foundation::{FreeLibrary, GetLastError, LocalFree};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
    FormatMessageW,
};
use windows_sys::Win32::System::LibraryLoader::{
    LOAD_LIBRARY_AS_DATAFILE, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LoadLibraryExW,
    SetDefaultDllDirectories,
};

#[derive(Debug)]
pub enum PluginLoadError {
    CannotOpenAsDataFile(String),
    LoadFailed(String),
    IoError(std::io::Error),
    InvalidPeFormat(String),
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginLoadError::CannotOpenAsDataFile(err) => {
                write!(f, "Cannot open as data file: {}", err)
            }
            PluginLoadError::LoadFailed(err) => write!(f, "Load failed: {}", err),
            PluginLoadError::IoError(err) => write!(f, "IO error: {}", err),
            PluginLoadError::InvalidPeFormat(err) => write!(f, "Invalid PE format: {}", err),
        }
    }
}

impl std::error::Error for PluginLoadError {}

pub fn load_dll(path: &Path) -> Result<Library, PluginLoadError> {
    let wide_path = utf16_path(path);

    unsafe {
        SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR);

        let handle = LoadLibraryExW(wide_path.as_ptr(), null_mut(), LOAD_LIBRARY_AS_DATAFILE);
        if handle.is_null() {
            return Err(PluginLoadError::CannotOpenAsDataFile(last_error_message()));
        } else {
            FreeLibrary(handle);
        }
    }

    unsafe { Library::new(path).map_err(|_| PluginLoadError::LoadFailed(last_error_message())) }
}

fn utf16_path(path: &Path) -> Vec<u16> {
    path.as_os_str().encode_wide().chain(Some(0)).collect()
}

fn last_error_message() -> String {
    unsafe {
        let error_code = GetLastError();
        let mut buffer: *mut u16 = null_mut();

        let len = FormatMessageW(
            FORMAT_MESSAGE_ALLOCATE_BUFFER
                | FORMAT_MESSAGE_FROM_SYSTEM
                | FORMAT_MESSAGE_IGNORE_INSERTS,
            null_mut(),
            error_code,
            0,
            std::mem::transmute(&mut buffer),
            0,
            null_mut(),
        );

        if len == 0 || buffer.is_null() {
            return format!("OS Error {}", error_code);
        }

        let message = String::from_utf16_lossy(std::slice::from_raw_parts(buffer, len as usize));
        LocalFree(buffer as _);
        message.trim().to_string()
    }
}
