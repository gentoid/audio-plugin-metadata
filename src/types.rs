use std::{ffi::c_void, path::PathBuf};

use vst2_sys::{AEffect, HostCallbackProc};

use crate::vst3::types::Vst3Info;

pub type Vst2IntPtr = isize;
pub type Vst2Main = unsafe extern "C" fn(callback: HostCallbackProc) -> *mut AEffect;
pub type Vst3Main = unsafe extern "system" fn() -> *mut c_void;

#[derive(Debug)]
pub enum PluginInfo {
    Vst2(Vst2Info),
    Vst3(Vst3Info),
}

#[derive(Debug)]
pub struct Vst2Info {
    pub path: PathBuf,
    pub name: Option<String>,
    pub vendor: Option<String>,
    pub version: u32,
    pub unique_id: u32,
    pub category: Vst2Category,
    pub category_raw: i32,
}

#[derive(Debug)]
pub enum Vst2Category {
    Unknown = vst2_sys::plug_category::UNKNOWN as isize,
    Effect = vst2_sys::plug_category::EFFECT as isize,
    Synth = vst2_sys::plug_category::SYNTH as isize,
    Analysis = vst2_sys::plug_category::ANALYSIS as isize,
    Mastering = vst2_sys::plug_category::MASTERING as isize,
    Spacializer = vst2_sys::plug_category::SPACIALIZER as isize,
    RoomFx = vst2_sys::plug_category::ROOM_FX as isize,
    SurroundFx = vst2_sys::plug_category::SURROUND_FX as isize,
    Restoration = vst2_sys::plug_category::RESTORATION as isize,
    OfflineProcess = vst2_sys::plug_category::OFFLINE_PROCESS as isize,
    Shell = vst2_sys::plug_category::SHELL as isize,
    Generator = vst2_sys::plug_category::GENERATOR as isize,
    MaxCount = vst2_sys::plug_category::MAX_COUNT as isize,
}

impl Vst2Category {
    pub fn from_num(num: i32) -> Vst2Category {
        use vst2_sys::plug_category::*;

        match num {
            EFFECT => Vst2Category::Effect,
            SYNTH => Vst2Category::Synth,
            ANALYSIS => Vst2Category::Analysis,
            MASTERING => Vst2Category::Mastering,
            SPACIALIZER => Vst2Category::Spacializer,
            ROOM_FX => Vst2Category::RoomFx,
            SURROUND_FX => Vst2Category::SurroundFx,
            RESTORATION => Vst2Category::Restoration,
            OFFLINE_PROCESS => Vst2Category::OfflineProcess,
            SHELL => Vst2Category::Shell,
            GENERATOR => Vst2Category::Generator,
            MAX_COUNT => Vst2Category::MaxCount,
            _ => Vst2Category::Unknown,
        }
    }
}
