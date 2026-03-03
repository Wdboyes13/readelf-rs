use crate::elf::types::*;

 pub struct Elf32Phdr {
     pub p_type: Elf32Word,
     pub p_offset: Elf32Off,
     pub p_vaddr: Elf32Addr,
     pub p_paddr: Elf32Addr,
     pub p_filesz: Elf32Word,
     pub p_memsz: Elf32Word,
     pub p_flags: Elf32Word,
     pub p_align: Elf32Word
 }

 pub struct Elf64Phdr {
     pub p_type: Elf64Word,
     pub p_flags: Elf64Word,
     pub p_offset: Elf64Off,
     pub p_vaddr: Elf64Addr,
     pub p_paddr: Elf64Addr,
     pub p_filsz: Elf64Xword,
     pub p_memsz: Elf64Xword,
     pub p_align: Elf64Xword
 }

pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;

pub const PF_X: u32 = 0x1;
pub const PF_W: u32 = 0x2;
pub const PF_R: u32 = 0x4;
