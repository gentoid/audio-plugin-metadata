#[derive(Debug)]
pub enum PluginFormat {
    Vst2,
}

#[derive(Debug)]
pub struct PluginInfo {
    pub path: String,
    pub name: Option<String>,
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub unique_id: u32,
    pub inputs: u32,
    pub outputs: u32,
    pub parameters: u32,
    pub presets: u32,
    pub is_synth: bool,
    pub has_editor: bool,
    pub category: Option<String>,
    pub format: PluginFormat,
}
