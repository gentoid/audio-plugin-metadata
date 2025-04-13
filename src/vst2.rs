use std::{ffi::CStr, os::raw::c_void, path::Path, ptr::NonNull};

use libloading::{Library, Symbol};
use vst2_sys::{AEffect, HostCallbackProc, effect_opcodes as opcode};

use crate::types::{PluginFormat, PluginInfo};

type VstIntPtr = isize;
type VstMain = unsafe extern "C" fn(callback: HostCallbackProc) -> *mut AEffect;

extern "C" fn dummy_host_callback(
    _effect: *mut AEffect,
    opcode: i32,
    _index: i32,
    _value: VstIntPtr,
    ptr: *mut c_void,
    _opt: f32,
) -> VstIntPtr {
    println!("Passed opcode: {opcode}");
    match opcode {
        1 => 2100, // audioMasterVersion
        33 => {
            // audioMasterGetVendorString
            let s = b"MyHost\0";
            unsafe {
                std::ptr::copy_nonoverlapping(s.as_ptr(), ptr as *mut u8, s.len());
            }
            1
        }
        34 => {
            // audioMasterGetProductString
            let s = b"MyHostProduct\0";
            unsafe {
                std::ptr::copy_nonoverlapping(s.as_ptr(), ptr as *mut u8, s.len());
            }
            1
        }
        35 => 1000, // audioMasterGetVendorVersion
        _ => 0,
    }
}

pub fn scan_vst2(path: &Path) -> Result<PluginInfo, Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new(path) }?;

    let vst_main: Symbol<VstMain> =
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
    let vendor = get_string(eff, opcode::GET_VENDOR_STRING);
    let version = get_string(eff, opcode::GET_VENDOR_VERSION);
    let category = get_string(eff, opcode::GET_PLUG_CATEGORY);

    Ok(PluginInfo {
        path: path.display().to_string(),
        name,
        vendor,
        version,
        category,
        unique_id: eff.unique_id as u32,
        inputs: eff.num_inputs as u32,
        outputs: eff.num_outputs as u32,
        parameters: eff.num_params as u32,
        presets: eff.num_programs as u32,
        is_synth: eff.flags & (1 << 8) != 0,
        has_editor: eff.flags & (1 << 0) != 0,
        format: PluginFormat::Vst2,
    })
}

fn get_string(eff: &AEffect, opcode: i32) -> Option<String> {
    let mut buffer = [0i8; 64];
    let dispatcher = eff.dispatcher;

    let result: VstIntPtr = dispatcher(
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
