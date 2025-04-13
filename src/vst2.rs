use std::{ffi::CStr, os::raw::c_void, path::Path, ptr::NonNull};

use libloading::{Library, Symbol};
use vst2_sys::AEffect;

use crate::types::{PluginFormat, PluginInfo, VstDispatcherOpcode as opcode};

type VstIntPtr = isize;
type HostCallback =
    unsafe extern "C" fn(*mut AEffect, i32, i32, VstIntPtr, *mut c_void, f32) -> VstIntPtr;
type VstMain = unsafe extern "C" fn(callback: HostCallback) -> *mut AEffect;

unsafe extern "C" fn dummy_host_callback(
    _effect: *mut AEffect,
    _opcode: i32,
    _index: i32,
    _value: VstIntPtr,
    _ptr: *mut c_void,
    _opt: f32,
) -> VstIntPtr {
    0
}

pub fn scan_vst2(path: &Path) -> Result<PluginInfo, Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new(path) }?;

    let vst_main: Symbol<VstMain> =
        unsafe { lib.get(b"VSTPluginMain").or_else(|_| lib.get(b"main"))? };

    let effect = unsafe { vst_main(dummy_host_callback) };
    let effect = NonNull::new(effect).ok_or("effect is null")?;
    let eff = unsafe { effect.as_ref() };

    let name =
        get_string(eff, opcode::EffGetEffectName as i32).ok_or("Plugin doesn't have name")?;
    let vendor = get_string(eff, opcode::EffGetVendorString as i32);
    let product = get_string(eff, opcode::EffGetProductString as i32);
    let version = get_string(eff, opcode::EffGetVendorVersion as i32);
    let category = get_string(eff, opcode::EffGetPlugCategory as i32);

    Ok(PluginInfo {
        path: path.display().to_string(),
        name,
        vendor,
        version,
        product,
        unique_id: Some(eff.unique_id as u32),
        inputs: Some(eff.num_inputs as u32),
        outputs: Some(eff.num_outputs as u32),
        parameters: Some(eff.num_params as u32),
        presets: Some(eff.num_programs as u32),
        is_synth: Some(eff.flags & (1 << 8) != 0),
        has_editor: Some(eff.flags & (1 << 0) != 0),
        category,
        format: PluginFormat::Vst2,
    })
}
fn get_string(eff: &AEffect, opcode: i32) -> Option<String> {
    let mut buffer = [0i8; 256];
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

    Some(
        unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned(),
    )
}
