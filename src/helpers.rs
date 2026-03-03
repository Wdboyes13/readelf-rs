use std::ffi::CStr;

pub fn truncate(s: &str, max: usize) -> &str {
    &s[..max.min(s.len())]
}

pub fn get_from_strtab(strndx: u32, strtab: &[u8]) -> String {
    return CStr::from_bytes_until_nul(&strtab[strndx as usize..])
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}
