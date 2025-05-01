use crate::utils::{i8_to_string, i16_to_string};
use libloading::Library;
use std::{error::Error, mem::MaybeUninit, path::Path};
use types::{
    ClassFlags, ClassInfo1, ClassInfo2, ClassInfo3, ClassesInfo, FactoryFlags, FactoryInfo, IID,
    Vst3Info, Vst3Main,
};
use vst3_sys::{
    VstPtr,
    base::{
        IPluginFactory, IPluginFactory2, IPluginFactory3, PClassInfo, PClassInfo2, PClassInfoW,
        PFactoryInfo, kResultOk,
    },
};

pub mod types;

pub fn scan_vst3(path: &Path) -> Result<Vst3Info, Box<dyn Error>> {
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
    let classes = scan_classes(factory)?;

    Ok(Vst3Info {
        factory_info,
        classes,
    })
}

fn read_factory_info(factory: &VstPtr<dyn IPluginFactory>) -> Result<FactoryInfo, Box<dyn Error>> {
    let mut info = MaybeUninit::<PFactoryInfo>::uninit();
    let res = unsafe { factory.get_factory_info(info.as_mut_ptr()) };

    if res != kResultOk {
        return Err("get_factory_info failed".into());
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

    if flags >= 8 {
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

fn read_class_flags(mut flags: u32) -> Vec<ClassFlags> {
    let mut classes = vec![];

    let all_flags = [
        ClassFlags::IsSynth,
        ClassFlags::IsEffect,
        ClassFlags::Undef,
        ClassFlags::PluginDoesMidi,
        ClassFlags::PluginDoesAudio,
        ClassFlags::NoAudioIO,
        ClassFlags::NeedMidiInput,
        ClassFlags::NeedMidiOutput,
    ];

    for flag in all_flags {
        if get_bit_and_shift(&mut flags) {
            classes.push(flag);
        }
    }

    classes
}

// mut u32
fn get_bit_and_shift(flags: &mut u32) -> bool {
    let bit = (*flags) & 1 == 1;
    (*flags) >>= 1;
    bit
}

fn scan_classes(factory: VstPtr<dyn IPluginFactory>) -> Result<ClassesInfo, Box<dyn Error>> {
    if let Some(factory) = factory.cast::<dyn IPluginFactory3>() {
        let classes = scan3(factory)?;
        return Ok(ClassesInfo::Classes3(classes));
    }

    if let Some(factory) = factory.cast::<dyn IPluginFactory2>() {
        let classes = scan2(factory)?;
        return Ok(ClassesInfo::Classes2(classes));
    }

    let classes = scan1(factory)?;
    Ok(ClassesInfo::Classes1(classes))
}

fn scan3(factory: VstPtr<dyn IPluginFactory3>) -> Result<Vec<ClassInfo3>, Box<dyn Error>> {
    let count = unsafe { factory.count_classes() };

    let mut classes = vec![];

    for i in 0..count {
        let mut info = MaybeUninit::<PClassInfoW>::uninit();
        let res = unsafe { factory.get_class_info_unicode(i, info.as_mut_ptr()) };

        if res != kResultOk {
            return Err(format!("Failed to get class info for {}", i).into());
        }

        let info = unsafe { info.assume_init() };

        let name = i16_to_string(&info.name);
        let category = i8_to_string(&info.category);
        let cardinality = info.cardinality;
        let cid = IID {
            data: info.cid.data,
        };
        let class_flags = read_class_flags(info.class_flags);
        let subcategories = i8_to_string(&info.subcategories)
            .split('|')
            .filter(|&s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect();

        let vendor = i16_to_string(&info.vendor);
        let version = i16_to_string(&info.version);
        let sdk_version = i16_to_string(&info.sdk_version);

        classes.push(ClassInfo3 {
            cid,
            cardinality,
            category,
            name,
            class_flags,
            subcategories,
            vendor,
            version,
            sdk_version,
        });
    }

    Ok(classes)
}

fn scan2(factory: VstPtr<dyn IPluginFactory2>) -> Result<Vec<ClassInfo2>, Box<dyn Error>> {
    let count = unsafe { factory.count_classes() };

    let mut classes = vec![];

    for i in 0..count {
        let mut info = MaybeUninit::<PClassInfo2>::uninit();
        let res = unsafe { factory.get_class_info2(i, info.as_mut_ptr()) };

        if res != kResultOk {
            return Err(format!("Failed to get class info for {}", i).into());
        }

        let info = unsafe { info.assume_init() };

        let name = i8_to_string(&info.name);
        let category = i8_to_string(&info.category);
        let cardinality = info.cardinality;
        let cid = IID {
            data: info.cid.data,
        };
        let class_flags = read_class_flags(info.class_flags);
        let subcategories = i8_to_string(&info.subcategories)
            .split('|')
            .filter(|&s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect();

        let vendor = i8_to_string(&info.vendor);
        let version = i8_to_string(&info.version);
        let sdk_version = i8_to_string(&info.sdk_version);

        classes.push(ClassInfo2 {
            cid,
            cardinality,
            category,
            name,
            class_flags,
            subcategories,
            vendor,
            version,
            sdk_version,
        });
    }

    Ok(classes)
}

fn scan1(factory: VstPtr<dyn IPluginFactory>) -> Result<Vec<ClassInfo1>, Box<dyn Error>> {
    let count = unsafe { factory.count_classes() };

    let mut classes = vec![];

    for i in 0..count {
        let mut info = MaybeUninit::<PClassInfo>::uninit();
        let res = unsafe { factory.get_class_info(i, info.as_mut_ptr()) };

        if res != kResultOk {
            return Err(format!("Failed to get class info for {}", i).into());
        }

        let info = unsafe { info.assume_init() };

        let name = i8_to_string(&info.name);
        let category = i8_to_string(&info.category);
        let cardinality = info.cardinality;
        let cid = IID {
            data: info.cid.data,
        };

        classes.push(ClassInfo1 {
            cid,
            cardinality,
            category,
            name,
        });
    }

    Ok(classes)
}
