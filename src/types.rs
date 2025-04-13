pub enum PluginFormat {
    Vst2,
}

pub struct PluginInfo {
    pub path: String,
    pub name: String,
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub product: Option<String>,
    pub unique_id: Option<u32>,
    pub inputs: Option<u32>,
    pub outputs: Option<u32>,
    pub parameters: Option<u32>,
    pub presets: Option<u32>,
    pub is_synth: Option<bool>,
    pub has_editor: Option<bool>,
    pub category: Option<String>,
    pub format: PluginFormat,
}

#[repr(i32)]
pub enum VstDispatcherOpcode {
    EffGetEffectName = 32,
    EffGetVendorString = 33,
    EffGetProductString = 34,
    EffGetVendorVersion = 35,
    EffGetPlugCategory = 36,
}
