use bincode::{
    BorrowDecode, Decode, Encode,
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
};
use serde::{Deserialize, Serialize};
use vst2_sys::{AEffect, HostCallbackProc};

pub type Vst2IntPtr = isize;
pub type Vst2Main = unsafe extern "C" fn(callback: HostCallbackProc) -> *mut AEffect;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Vst2Info {
    pub name: Option<String>,
    pub vendor: Option<String>,
    pub version: u32,
    pub unique_id: u32,
    pub category: Vst2Category,
    pub category_raw: i32,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Encode for Vst2Category {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        use vst2_sys::plug_category;

        let value: i32 = match self {
            Vst2Category::Unknown => plug_category::UNKNOWN,
            Vst2Category::Effect => plug_category::EFFECT,
            Vst2Category::Synth => plug_category::SYNTH,
            Vst2Category::Analysis => plug_category::ANALYSIS,
            Vst2Category::Mastering => plug_category::MASTERING,
            Vst2Category::Spacializer => plug_category::SPACIALIZER,
            Vst2Category::RoomFx => plug_category::ROOM_FX,
            Vst2Category::SurroundFx => plug_category::SURROUND_FX,
            Vst2Category::Restoration => plug_category::RESTORATION,
            Vst2Category::OfflineProcess => plug_category::OFFLINE_PROCESS,
            Vst2Category::Shell => plug_category::SHELL,
            Vst2Category::Generator => plug_category::GENERATOR,
            Vst2Category::MaxCount => plug_category::MAX_COUNT,
        };

        value.encode(encoder)
    }
}

impl<C> Decode<C> for Vst2Category {
    fn decode<D: Decoder<Context = C>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let value: i32 = Decode::decode(decoder)?;
        Ok(Vst2Category::from_num(value))
    }
}

impl<'a, 'de: 'a, Context> BorrowDecode<'de, Context> for Vst2Category {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        Vst2Category::decode(decoder)
    }
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
