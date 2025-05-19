use std::{
    ffi::CStr,
    os::raw::{c_char, c_void},
    panic::{AssertUnwindSafe, catch_unwind},
    path::Path,
    ptr::NonNull,
};

use libloading::Symbol;
use tracing::{debug, error};
use types::{Vst2Category, Vst2Info, Vst2IntPtr, Vst2Main};
use vst2_sys::{AEffect, effect_opcodes as opcode};

use crate::lib_loader::load_dll;

pub mod types;

extern "C" fn dummy_host_callback(
    _effect: *mut AEffect,
    opcode: i32,
    _index: i32,
    _value: Vst2IntPtr,
    ptr: *mut c_void,
    _opt: f32,
) -> Vst2IntPtr {
    match opcode {
        1 => 2100, // audioMasterVersion
        33 => {
            if ptr.is_null() {
                return 0;
            }

            // audioMasterGetVendorString
            let s = b"MyHost\0";
            unsafe {
                std::ptr::copy_nonoverlapping(s.as_ptr(), ptr as *mut u8, s.len().min(64));
            }
            1
        }
        34 => {
            if ptr.is_null() {
                return 0;
            }

            // audioMasterGetProductString
            let s = b"MyHostProduct\0";
            unsafe {
                std::ptr::copy_nonoverlapping(s.as_ptr(), ptr as *mut u8, s.len().min(64));
            }
            1
        }
        35 => 1000, // audioMasterGetVendorVersion
        vst2_sys::host_opcodes::CAN_DO => {
            if ptr.is_null() {
                return 0;
            }

            let result = catch_unwind(AssertUnwindSafe(|| {
                let cstr = unsafe { CStr::from_ptr(ptr as *const c_char) };
                debug!("Can do {:?}", cstr);

                if let Ok(text) = cstr.to_str() {
                    match text {
                        "sendVstTimeInfo" => 0,
                        _ => 0,
                    }
                } else {
                    0
                }
            }));

            match result {
                Ok(v) => v,
                Err(_) => {
                    error!("CAN_DO panic: plugin passed invalid pointer or string");
                    0
                }
            }
        }
        _ => {
            debug!("Unhandled host opcode: {}", opcode);
            0
        }
    }
}

pub fn scan_vst2(path: &Path) -> Result<Vst2Info, Box<dyn std::error::Error>> {
    let lib = load_dll(path)?;

    let vst_main: Symbol<Vst2Main> =
        unsafe { lib.get(b"VSTPluginMain").or_else(|_| lib.get(b"main"))? };

    let effect = unsafe { vst_main(dummy_host_callback) };
    let effect = NonNull::new(effect).ok_or("effect is null")?;
    let eff = unsafe { effect.as_ref() };

    ((eff.dispatcher)(
        eff as *const _ as *mut AEffect,
        opcode::OPEN,
        0,
        0,
        std::ptr::null_mut(),
        0.0,
    ));

    let name = get_string(eff, opcode::GET_EFFECT_NAME)
        .or_else(|| get_string(eff, opcode::GET_PRODUCT_STRING));
    let category_raw = get_num(eff, opcode::GET_PLUG_CATEGORY) as i32;

    let info = Vst2Info {
        name,
        vendor: get_string(eff, opcode::GET_VENDOR_STRING),
        version: eff.version as u32,
        category: Vst2Category::from_num(category_raw),
        category_raw,
        unique_id: eff.unique_id as u32,
    };

    ((eff.dispatcher)(
        eff as *const _ as *mut AEffect,
        opcode::CLOSE,
        0,
        0,
        std::ptr::null_mut(),
        0.0,
    ));

    Ok(info)
}

fn get_string(eff: &AEffect, opcode: i32) -> Option<String> {
    let mut buffer = [0i8; 64];
    let dispatcher = eff.dispatcher;

    let result = dispatcher(
        eff as *const _ as *mut AEffect,
        opcode,
        0,
        0,
        buffer.as_mut_ptr() as *mut c_void,
        0.0,
    );

    if result == 0 {
        return None;
    }

    let string = unsafe { CStr::from_ptr(buffer.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    if string.is_empty() {
        return None;
    }

    Some(string)
}

fn get_num(eff: &AEffect, opcode: i32) -> Vst2IntPtr {
    let dispatcher = eff.dispatcher;

    dispatcher(
        eff as *const _ as *mut AEffect,
        opcode,
        0,
        0,
        std::ptr::null_mut(),
        0.0,
    )
}
