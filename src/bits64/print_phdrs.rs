use crate::elf::*;
use crate::helpers::{get_elf_type64, off_to_str};
use indoc::indoc;

fn ptype_str(p_type: u32) -> String {
    match p_type {
        PT_NULL => "NULL",
        PT_LOAD => "LOAD",
        PT_DYNAMIC => "DYNAMIC",
        PT_INTERP => "INTERP",
        PT_NOTE => "NOTE",
        PT_SHLIB => "SHLIB",
        PT_PHDR => "PHDR",
        PT_TLS => "TLS",
        _ => "UNKNOWN",
    }
    .to_string()
}

fn pflags_str(flags: u32) -> String {
    let mut fstr = String::new();
    if flags & PF_R != 0 {
        fstr.push('R');
    } else {
        fstr.push(' ');
    }

    if flags & PF_W != 0 {
        fstr.push('W');
    } else {
        fstr.push(' ');
    }

    if flags & PF_X != 0 {
        fstr.push('E');
    } else {
        fstr.push(' ');
    }

    fstr
}

fn print_phdr_info(phdr: &Elf64Phdr) {
    println!(
        indoc! {"
          {:<14} {:#016x} {:#016x} {:#016x}
                         {:#016x} {:#016x}   {:<3}    {:#x}"},
        ptype_str(phdr.p_type),
        phdr.p_offset,
        phdr.p_vaddr,
        phdr.p_paddr,
        phdr.p_filsz,
        phdr.p_memsz,
        pflags_str(phdr.p_flags),
        phdr.p_align
    );
}

pub fn print_phdrs(ehdr: &Elf64Ehdr, phdrs: &[Elf64Phdr], data: &[u8]) {
    println!("\nElf file type is {}", get_elf_type64(ehdr));
    println!("Entry point {:#x}", ehdr.e_entry);
    println!(
        "There are {} program headers, starting at offset {}",
        ehdr.e_phnum, ehdr.e_phoff
    );

    println!(indoc! {"
    Program Headers:
      Type           Offset             VirtAddr           PhysAddr
                     FileSiz            MemSiz              Flags  Align
    "});

    for phdr in phdrs {
        print_phdr_info(phdr);
        if phdr.p_type == PT_INTERP {
            println!(
                "      [Requesting program interpreter: {}]",
                off_to_str(phdr.p_offset as usize, data)
            );
        }
    }
}
