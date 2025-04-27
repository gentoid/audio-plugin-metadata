use std::ffi::CStr;

pub fn i8_to_string(data: &[i8]) -> String {
    unsafe { CStr::from_ptr(data.as_ptr()) }
        .to_string_lossy()
        .into()
}

pub fn i16_to_string(data: &[i16]) -> String {
    let mut vector: Vec<u16> = data.iter().map(|&byte| byte as u16).collect();

    if let Some(index) = vector.iter().position(|&x| x == 0) {
        vector.truncate(index);
    }

    String::from_utf16_lossy(&vector)
}
