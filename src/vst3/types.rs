use vst3_sys::sys::GUID;

pub type IID = GUID;

#[derive(Debug)]
pub struct Info {
    pub factory_info: FactoryInfo,
    pub classes: ClassesInfo,
}

#[derive(Debug)]
pub enum FactoryFlags {
    ClassesDiscardable,      //  1
    LicenseCheck,            //  2
    ComponentNonDiscardable, //  8
    Unicode,                 // 16
}

#[derive(Debug)]
pub enum ClassFlags {
    IsSynth,         // 0x01
    IsEffect,        // 0x02
    Undef,           // 0x04
    PluginDoesMidi,  // 0x08
    PluginDoesAudio, // 0x10
    NoAudioIO,       // 0x20
    NeedMidiInput,   // 0x40
    NeedMidiOutput,  // 0x80
}

#[derive(Debug)]
pub struct FactoryInfo {
    pub vendor: String, // [char8; 64]
    pub url: String,    // [char8; 256]
    pub email: String,  // [char8; 128]
    pub flags: Vec<FactoryFlags>,
}

#[derive(Debug)]
pub enum ClassCardinality {
    ManyInstances = 0x7FFF_FFFF,
}

#[derive(Debug)]
pub enum ClassesInfo {
    Classes1(Vec<ClassInfo1>),
    Classes2(Vec<ClassInfo2>),
    Classes3(Vec<ClassInfo3>),
}

#[derive(Debug)]
pub struct ClassInfo1 {
    pub cid: IID,
    pub cardinality: i32,
    pub category: String, // [char8; 32]
    pub name: String,     // [char8; 64]
}

#[derive(Debug)]
pub struct ClassInfo2 {
    pub cid: IID,
    pub cardinality: i32,
    pub category: String, // [char8; 32]
    pub name: String,     // [char8; 64]
    pub class_flags: Vec<ClassFlags>,
    pub subcategories: Vec<String>, // [char8; 128]
    pub vendor: String,             // [char8; 64]
    pub version: String,            // [char8; 64]
    pub sdk_version: String,        // [char8; 64]
}

#[derive(Debug)]
pub struct ClassInfo3 {
    pub cid: IID,
    pub cardinality: i32,
    pub category: String, // [char8; 32]
    pub name: String,     // [char16; 64]
    pub class_flags: Vec<ClassFlags>,
    pub subcategories: Vec<String>, // [char8; 128]
    pub vendor: String,             // [char16; 64]
    pub version: String,            // [char16; 64]
    pub sdk_version: String,        // [char16; 64]
}
