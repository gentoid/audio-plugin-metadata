use vst3_sys::sys::GUID;

pub type IID = GUID;

#[derive(Debug)]
pub enum FactoryFlags {
    NoFlags,                 //  0
    ClassesDiscardable,      //  1
    LicenseCheck,            //  2
    ComponentNonDiscardable, //  8
    Unicode,                 // 16
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
pub struct ClassInfo {
    pub cid: IID,
    pub cardinality: ClassCardinality,
    pub category: String, // [char8; 32]
    pub name: String,     // [char8; 64]
}

#[derive(Debug)]
pub struct ClassInfo2 {
    pub cid: IID,
    pub cardinality: ClassCardinality,
    pub category: String, // [char8; 32]
    pub name: String,     // [char8; 64]
    pub class_flags: u32,
    pub subcategories: String, // [char8; 128]
    pub vendor: String,        // [char8; 64]
    pub version: String,       // [char8; 64]
    pub sdk_version: String,   // [char8; 64]
}

#[derive(Debug)]
pub struct ClassInfo3 {
    pub cid: IID,
    pub cardinality: ClassCardinality,
    pub category: String, // [char8; 32]
    pub name: String,     // [char16; 64]
    pub class_flags: u32,
    pub subcategories: String, // [char8; 128]
    pub vendor: String,        // [char16; 64]
    pub version: String,       // [char16; 64]
    pub sdk_version: String,   // [char16; 64]
}
