use std::{error::Error, mem::MaybeUninit, path::Path};

use libloading::Library;
use types::{FactoryFlags, FactoryInfo};
use vst3_sys::{
    VstPtr,
    base::{
        IPluginFactory, IPluginFactory2, IPluginFactory3, PClassInfo, PClassInfo2, PClassInfoW,
        PFactoryInfo, kResultOk,
    },
};

use crate::{
    types::Vst3Main,
    utils::{i8_to_string, i16_to_string},
};

pub mod types;

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
    let factory_info = read_factory_info(&factory)?;

    println!("factory info = {:#?}", factory_info);

    scan_classes(factory)
}

fn read_factory_info(factory: &VstPtr<dyn IPluginFactory>) -> Result<FactoryInfo, Box<dyn Error>> {
    let mut info = MaybeUninit::<PFactoryInfo>::uninit();
    let res = unsafe { factory.get_factory_info(info.as_mut_ptr()) };

    if res != kResultOk {
        return Err(format!("get_factory_info failed").into());
    }

    let info = unsafe { info.assume_init() };

    let vendor = i8_to_string(&info.vendor);
    let url = i8_to_string(&info.url);
    let email = i8_to_string(&info.email);

    Ok(FactoryInfo {
        vendor,
        url,
        email,
        flags: read_flags(info.flags),
    })
}

fn read_flags(mut flags: i32) -> Vec<FactoryFlags> {
    let mut res = vec![];

    if flags >= 32 {
        flags = 0;
    }

    if flags >= 16 {
        res.push(FactoryFlags::Unicode);
        flags -= 16;
    }

    if flags >= 8  {
        res.push(FactoryFlags::ComponentNonDiscardable);
        flags -= 8;
    }

    if flags >= 2 {
        res.push(FactoryFlags::LicenseCheck);
        flags -= 2;
    }

    if flags >= 1 {
        res.push(FactoryFlags::ClassesDiscardable);
    }

    res
}

fn scan_classes(factory: VstPtr<dyn IPluginFactory>) -> Result<(), Box<dyn std::error::Error>> {
    let count = unsafe { factory.count_classes() };

    println!("Found {count} class(es):");

    if let Some(factory) = factory.cast::<dyn IPluginFactory3>() {
        return scan3(factory);
    }

    if let Some(factory) = factory.cast::<dyn IPluginFactory2>() {
        return scan2(factory);
    }

    scan1(factory)
}

fn scan3(factory: VstPtr<dyn IPluginFactory3>) -> Result<(), Box<dyn Error>> {
    let count = unsafe { factory.count_classes() };

    for i in 0..count {
        let mut info = MaybeUninit::<PClassInfoW>::uninit();
        let res = unsafe { factory.get_class_info_unicode(i, info.as_mut_ptr()) };

        if res != kResultOk {
            return Err(format!("Failed to get class info for {}", i).into());
        }

        let info = unsafe { info.assume_init() };
        let name = i16_to_string(&info.name);
        println!(
            "Class {} (Factory3): {:?} (category: {:?})",
            i, name, info.class_flags
        );
    }

    Ok(())
}

fn scan2(factory: VstPtr<dyn IPluginFactory2>) -> Result<(), Box<dyn Error>> {
    let count = unsafe { factory.count_classes() };

    for i in 0..count {
        let mut info = MaybeUninit::<PClassInfo2>::uninit();
        let res = unsafe { factory.get_class_info2(i, info.as_mut_ptr()) };

        if res != kResultOk {
            return Err(format!("Failed to get class info for {}", i).into());
        }

        let info = unsafe { info.assume_init() };

        let name = i8_to_string(&info.name);
        let version = i8_to_string(&info.version);
        let subcategories = i8_to_string(&info.subcategories);

        println!(
            "Class {} (Factory2): {} (version: {}, subcategories: {})",
            i, name, version, subcategories,
        );
    }

    Ok(())
}

fn scan1(factory: VstPtr<dyn IPluginFactory>) -> Result<(), Box<dyn Error>> {
    let count = unsafe { factory.count_classes() };

    for i in 0..count {
        let mut info = MaybeUninit::<PClassInfo>::uninit();
        let res = unsafe { factory.get_class_info(i, info.as_mut_ptr()) };

        if res != kResultOk {
            return Err(format!("Failed to get class info for {}", i).into());
        }

        let info = unsafe { info.assume_init() };

        let name = i8_to_string(&info.name);
        let category = i8_to_string(&info.category);

        println!("Class {} (Factory1): {} (category: {})", i, name, category,);
    }

    Ok(())
}
