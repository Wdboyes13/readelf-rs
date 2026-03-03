use crate::helpers::*;
use crate::elf::*;
use indoc::indoc;

fn shdr_type_text(shdr: &Elf64Shdr) -> &str {
    match shdr.sh_type {
        SHT_NULL => "NULL",
        SHT_PROGBITS => "PROGBITS",
        SHT_SYMTAB => "SYMTAB",
        SHT_STRTAB => "STRTAB",
        SHT_RELA => "RELA",
        SHT_HASH => "HASH",
        SHT_DYNAMIC => "DYNAMIC",
        SHT_NOTE => "NOTE",
        SHT_NOBITS => "NOBITS",
        SHT_REL => "REL",
        SHT_SHLIB => "SHLIB",
        SHT_DYNSYM => "DYNSYM",
        SHT_INIT_ARRAY => "INIT_ARRAY",
        SHT_FINI_ARRAY => "FINI_ARRAY",
        SHT_PREINIT_ARRAY => "PREINIT_ARRAY",
        SHT_GROUP => "GROUP",
        SHT_SYMTAB_SHNDX => "SYMTAB_SHNDX",
        _ => "UNKNOWN"
    }
}

pub fn get_shname(shdr: &Elf64Shdr, shstrtab: &[u8]) -> String {
    return get_from_strtab(shdr.sh_name, shstrtab);
}

pub fn get_flagstr(flags: Elf64Xword) -> String {
    const FLAG_MAP: &[(u64, char)] = &[
        (SHF_WRITE as u64,          'W'),
        (SHF_ALLOC as u64,          'A'),
        (SHF_EXECINSTR as u64,      'X'),
        (SHF_MERGE as u64,          'M'),
        (SHF_STRINGS as u64,        'S'),
        (SHF_INFO_LINK as u64,      'I'),
        (SHF_LINK_ORDER as u64,     'L'),
        (SHF_OS_NONCONFORMING as u64, 'O'),
        (SHF_GROUP as u64,          'G'),
        (SHF_TLS as u64,            'T')
    ];

    FLAG_MAP.iter()
        .filter(|&&(flag, _)| flags & flag != 0)
        .map(|&(_, ch) | ch)
        .collect()
}

pub fn print_shdrs(ehdr: &Elf64Ehdr, shdrs: &[Elf64Shdr], shstrtab: &[u8]) {

    println!("There are {} section headers, starting at offset 0x{:x}:\n", ehdr.e_shnum, ehdr.e_shoff);
    println!(indoc!{"
        Section Headers:
          [Nr] Name              Type             Address           Offset
               Size              EntSize          Flags  Link  Info  Align
    "});

    for i in 0..=(ehdr.e_shnum - 1) {
        let tshdr = shdrs[i as usize];
        println!(
            "  [{:2}] {:17} {:16} {:016x} {:08x}",
            i,
            truncate(get_shname(&tshdr, shstrtab).as_str(), 17),
            truncate(shdr_type_text(&tshdr), 16),
            tshdr.sh_addr,
            tshdr.sh_offset
        );
        println!(
            "       {:016x}  {:016x} {:6} {:6} {:6} {:4}",
            tshdr.sh_size,
            tshdr.sh_entsize,
            get_flagstr(tshdr.sh_flags),
            tshdr.sh_link,
            tshdr.sh_info,
            tshdr.sh_addralign
        );
    }
    println!(indoc!{"
        Key to Flags:
          W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
          L (link order), O (extra OS processing required), G (group), T (TLS)
    "});
}
