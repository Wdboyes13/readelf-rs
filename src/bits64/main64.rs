use crate::elf;
use crate::parser::Args;
use crate::bits64::{
    print_ehdr::print_ehdr,
    print_shdrs::print_shdrs,
    print_phdrs::print_phdrs
};

pub fn main64(data: &Vec<u8>, args: &Args) {
    let ehdr: elf::Elf64Ehdr = unsafe { std::ptr::read(data.as_ptr() as *const elf::Elf64Ehdr) };

    let shdr_start = ehdr.e_shoff as usize;
    let shdr_end = shdr_start + size_of::<elf::Elf64Shdr>() * ehdr.e_shnum as usize;
    let shdrs: &[elf::Elf64Shdr] = unsafe {
        std::slice::from_raw_parts(
            data[shdr_start..shdr_end].as_ptr() as *const elf::Elf64Shdr,
            ehdr.e_shnum as usize,
        )
    };

    let shstrtab_offset = shdrs[ehdr.e_shstrndx as usize].sh_offset as usize;
    let shstrtab_size = shdrs[ehdr.e_shstrndx as usize].sh_size as usize;
    let shstrtab = &data[shstrtab_offset..shstrtab_offset + shstrtab_size];

    let phdr_start = ehdr.e_phoff as usize;
    let phdr_end = phdr_start + size_of::<elf::Elf64Phdr>() * ehdr.e_phnum as usize;
    let phdrs: &[elf::Elf64Phdr] = unsafe {
        std::slice::from_raw_parts(
            data[phdr_start..phdr_end].as_ptr() as *const elf::Elf64Phdr,
            ehdr.e_phnum as usize
        )
    };

    if args.ehdr {
        print_ehdr(&ehdr);
    }

    if args.shdr {
        print_shdrs(&ehdr, shdrs, shstrtab);
    }

    if args.phdr {
        print_phdrs(&ehdr, phdrs);
    }
}
