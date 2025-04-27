use std::{ffi::CStr, path::Path};

use libloading::Library;
use vst3_sys::{
    VstPtr,
    base::{IPluginFactory, PClassInfo, PFactoryInfo, kResultOk},
};

use crate::types::Vst3Main;

pub fn scan_vst3(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new(path) }?;

    let get_factory: libloading::Symbol<Vst3Main> = unsafe { lib.get(b"GetPluginFactory\0") }?;
    let factory_ptr = unsafe { get_factory() };

    if factory_ptr.is_null() {
        return Err("PluginFactory is null".into());
    }

    let factory = unsafe {
        VstPtr::<dyn IPluginFactory>::owned(factory_ptr as *mut *mut _)
            .ok_or("Failed to create VstPtr for factory")?
    };
    let mut f_info: PFactoryInfo = unsafe { std::mem::zeroed() };
    let res = unsafe { factory.get_factory_info(&mut f_info) };

    if res != kResultOk {
        return Err(format!("get_factory_info failed").into());
    }

    let vendor = unsafe { CStr::from_ptr(f_info.vendor.as_ptr()).to_string_lossy() };
    let url = unsafe { CStr::from_ptr(f_info.url.as_ptr()).to_string_lossy() };
    let email = unsafe { CStr::from_ptr(f_info.email.as_ptr()).to_string_lossy() };

    println!("vendor = {}, url = {}, email = {}", vendor, url, email);

    let count = unsafe { factory.count_classes() };

    for i in 0..count {
        let mut info: PClassInfo = unsafe { std::mem::zeroed() };
        let result = unsafe { factory.get_class_info(i, &mut info) };

        if result != kResultOk {
            return Err(format!("get_class_info failed for index {}", i).into());
        }

        let name = unsafe { CStr::from_ptr(info.name.as_ptr()).to_string_lossy() };
        let category = unsafe { CStr::from_ptr(info.category.as_ptr()).to_string_lossy() };
        let cardinality = info.cardinality;
        let cid = info.cid;

        println!(
            "Plugin {}: name = {}, category = {}, cardinality = {}, cid = {:?}",
            i, name, category, cardinality, cid
        );
    }

    Ok(())
}
