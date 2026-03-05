use crate::elf::*;
use std::ffi::CStr;

pub fn truncate(s: &str, max: usize) -> &str {
    &s[..max.min(s.len())]
}

pub fn off_to_str(offset: usize, data: &[u8]) -> String {
    return CStr::from_bytes_until_nul(&data[offset..])
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn get_from_strtab(strndx: u32, strtab: &[u8]) -> String {
    return off_to_str(strndx as usize, strtab);
}

pub fn get_elf_type64(ehdr: &Elf64Ehdr) -> String {
    match ehdr.e_type {
        ET_NONE => "NONE (No file type)",
        ET_REL => "REL (Relocatable file)",
        ET_EXEC => "EXEC (Executable file)",
        ET_DYN => "DYN (Shared object file)",
        ET_CORE => "CORE (Core file)",
        _ => "unknown",
    }
    .to_string()
}

pub fn get_elf_type32(ehdr: &Elf32Ehdr) -> String {
    match ehdr.e_type {
        ET_NONE => "NONE (No file type)",
        ET_REL => "REL (Relocatable file)",
        ET_EXEC => "EXEC (Executable file)",
        ET_DYN => "DYN (Shared object file)",
        ET_CORE => "CORE (Core file)",
        _ => "unknown",
    }
    .to_string()
}
