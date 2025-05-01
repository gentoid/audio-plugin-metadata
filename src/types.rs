use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{vst2::types::Vst2Info, vst3::types::Vst3Info};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum PluginInfo {
    Vst2(Vst2Info),
    Vst3(Vst3Info),
}
