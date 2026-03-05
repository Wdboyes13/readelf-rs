use crate::elf::types::*;

const EI_NIDENT: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32Ehdr {
    pub e_ident: [Uchar; EI_NIDENT],
    pub e_type: Elf32Half,
    pub e_machine: Elf32Half,
    pub e_version: Elf32Word,
    pub e_entry: Elf32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
    pub e_flags: Elf32Word,
    pub e_ehsize: Elf32Half,
    pub e_phentsize: Elf32Half,
    pub e_phnum: Elf32Half,
    pub e_shentsize: Elf32Half,
    pub e_shnum: Elf32Half,
    pub e_shstrndx: Elf32Half,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64Ehdr {
    pub e_ident: [Uchar; EI_NIDENT],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

pub const ET_NONE: u16 = 0;
pub const ET_REL: u16 = 1;
pub const ET_EXEC: u16 = 2;
pub const ET_DYN: u16 = 3;
pub const ET_CORE: u16 = 4;

pub const EV_NONE: u32 = 0;
pub const EV_CURRENT: u32 = 1;

pub const EI_MAG0: Uchar = 0;
pub const EI_MAG1: Uchar = 1;
pub const EI_MAG2: Uchar = 2;
pub const EI_MAG3: Uchar = 3;
pub const EI_CLASS: Uchar = 4;
pub const EI_DATA: Uchar = 5;
pub const EI_VERSION: Uchar = 6;
pub const EI_OSABI: Uchar = 7;
pub const EI_ABIVERSION: Uchar = 8;
pub const EI_PAD: Uchar = 9;

pub const ELFMAG0: Uchar = 0x7f;
pub const ELFMAG1: Uchar = 'E' as u8;
pub const ELFMAG2: Uchar = 'L' as u8;
pub const ELFMAG3: Uchar = 'F' as u8;

pub const ELFCLASSNONE: Uchar = 0;
pub const ELFCLASS32: Uchar = 1;
pub const ELFCLASS64: Uchar = 2;

pub const ELFDATANONE: Uchar = 0;
pub const ELFDATA2LSB: Uchar = 1;
pub const ELFDATA2MSB: Uchar = 2;
