use crate::elf::*;

fn osabi_text(ehdr: &Elf64Ehdr) -> &str {
    match ehdr.e_ident[EI_OSABI as usize] {
        ELFOSABI_NONE    => "UNIX - System V",
        ELFOSABI_HPUX    => "HP-UX",
        ELFOSABI_NETBSD  => "NetBSD",
        ELFOSABI_LINUX   => "Linux",
        ELFOSABI_SOLARIS => "Solaris",
        ELFOSABI_AIX     => "AIX",
        ELFOSABI_IRIX    => "IRIX",
        ELFOSABI_FREEBSD => "FreeBSD",
        ELFOSABI_TRU64   => "Compaq TRU64 UNIX",
        ELFOSABI_MODESTO => "Novell Modesto",
        ELFOSABI_OPENBSD => "OpenBSD",
        ELFOSABI_OPENVMS => "OpenVMS",
        ELFOSABI_NSK     => "HP Non-Stop Kernel",
        _                => "unknown",
    }
}

fn emach_text(ehdr: &Elf64Ehdr) -> &str {
    match ehdr.e_machine {
        EM_NONE        => "None",
        EM_M32         => "AT&T WE 32100",
        EM_SPARC       => "SPARC",
        EM_386         => "Intel 80386",
        EM_68K         => "Motorola 68000",
        EM_88K         => "Motorola 88000",
        EM_860         => "Intel 80860",
        EM_MIPS        => "MIPS I Architecture",
        EM_S370        => "IBM System/370",
        EM_MIPS_RS3_LE => "MIPS RS3000 Little-endian",
        EM_PARISC      => "HP PA-RISC",
        EM_VPP500      => "Fujitsu VPP500",
        EM_SPARC32PLUS => "Enhanced SPARC",
        EM_960         => "Intel 80960",
        EM_PPC         => "PowerPC",
        EM_PPC64       => "64-bit PowerPC",
        EM_S390        => "IBM System/390",
        EM_V800        => "NEC V800",
        EM_FR20        => "Fujitsu FR20",
        EM_RH32        => "TRW RH-32",
        EM_RCE         => "Motorola RCE",
        EM_ARM         => "ARM",
        EM_ALPHA       => "Digital Alpha",
        EM_SH          => "Hitachi SH",
        EM_SPARCV9     => "SPARC Version 9",
        EM_TRICORE     => "Siemens TriCore",
        EM_ARC         => "Argonaut RISC Core",
        EM_H8_300      => "Hitachi H8/300",
        EM_H8_300H     => "Hitachi H8/300H",
        EM_H8S         => "Hitachi H8S",
        EM_H8_500      => "Hitachi H8/500",
        EM_IA_64       => "Intel IA-64",
        EM_MIPS_X      => "Stanford MIPS-X",
        EM_COLDFIRE    => "Motorola ColdFire",
        EM_68HC12      => "Motorola M68HC12",
        EM_MMA         => "Fujitsu MMA",
        EM_PCP         => "Siemens PCP",
        EM_NCPU        => "Sony nCPU",
        EM_NDR1        => "Denso NDR1",
        EM_STARCORE    => "Motorola Star*Core",
        EM_ME16        => "Toyota ME16",
        EM_ST100       => "STMicroelectronics ST100",
        EM_TINYJ       => "Advanced Logic TinyJ",
        EM_X86_64      => "Advanced Micro Devices X86-64",
        EM_PDSP        => "Sony DSP Processor",
        EM_PDP10       => "DEC PDP-10",
        EM_PDP11       => "DEC PDP-11",
        EM_FX66        => "Siemens FX66",
        EM_ST9PLUS     => "STMicroelectronics ST9+",
        EM_ST7         => "STMicroelectronics ST7",
        EM_68HC16      => "Motorola MC68HC16",
        EM_68HC11      => "Motorola MC68HC11",
        EM_68HC08      => "Motorola MC68HC08",
        EM_68HC05      => "Motorola MC68HC05",
        EM_SVX         => "Silicon Graphics SVx",
        EM_ST19        => "STMicroelectronics ST19",
        EM_VAX         => "Digital VAX",
        EM_CRIS        => "Axis Communications 32-bit",
        EM_JAVELIN     => "Infineon Technologies 32-bit",
        EM_FIREPATH    => "Element 14 64-bit DSP",
        EM_ZSP         => "LSI Logic 16-bit DSP",
        EM_MMIX        => "Donald Knuth's MMIX",
        EM_HUANY       => "Harvard machine-independent",
        EM_PRISM       => "SiTera Prism",
        EM_AVR         => "Atmel AVR 8-bit",
        EM_FR30        => "Fujitsu FR30",
        EM_D10V        => "Mitsubishi D10V",
        EM_D30V        => "Mitsubishi D30V",
        EM_V850        => "NEC v850",
        EM_M32R        => "Mitsubishi M32R",
        EM_MN10300     => "Matsushita MN10300",
        EM_MN10200     => "Matsushita MN10200",
        EM_PJ          => "picoJava",
        EM_OPENRISC    => "OpenRISC 32-bit",
        EM_ARC_A5      => "ARC Cores Tangent-A5",
        EM_XTENSA      => "Tensilica Xtensa",
        EM_VIDEOCORE   => "Alphamosaic VideoCore",
        EM_TMM_GPP     => "Thompson Multimedia GPP",
        EM_NS32K       => "National Semiconductor 32000",
        EM_TPC         => "Tenor Network TPC",
        EM_SNP1K       => "Trebia SNP 1000",
        EM_ST200       => "STMicroelectronics ST200",
        EM_AARCH64     => "AArch64",
        _              => "unknown",
    }
}

pub fn print_ehdr(ehdr: &Elf64Ehdr) {
    print!("ELF Header:\n  Magic:   ");
    for byte in &ehdr.e_ident {
        print!("{:02x} ", byte);
    }
    println!();

    let class = match ehdr.e_ident[EI_CLASS as usize] {
        ELFCLASS32 => "ELF32",
        ELFCLASS64 => "ELF64",
        _ => "unknown",
    };
    println!("  {:<35} {}", "Class:", class);
    println!("  {:<35} {}", "Data:", match ehdr.e_ident[EI_DATA as usize] {
        ELFDATA2LSB => "2's complement, little endian",
        ELFDATA2MSB => "2's complement, big endian",
        _ => "unknown",
    });

    println!("  {:<35} {} (current)", "Version:", ehdr.e_ident[EI_VERSION as usize]);
    println!("  {:<35} {}", "OS/ABI:", osabi_text(ehdr));
    println!("  {:<35} {}", "ABI Version:", ehdr.e_ident[EI_ABIVERSION as usize]);
    println!("  {:<35} {}", "Type:", match ehdr.e_type {
        ET_NONE => "NONE (No file type)",
        ET_REL => "REL (Relocatable file)",
        ET_EXEC => "EXEC (Executable file)",
        ET_DYN => "DYN (Shared object file)",
        ET_CORE => "CORE (Core file)",
        _ => "unknown",
    });

    println!("  {:<35} {}", "Machine:", emach_text(ehdr));
    println!("  {:<35} {:#x}", "Version:", ehdr.e_version);
    println!("  {:<35} {:#x}", "Entry point address:", ehdr.e_entry);
    println!("  {:<35} {} (bytes into file)", "Start of program headers:", ehdr.e_phoff);
    println!("  {:<35} {} (bytes into file)", "Start of section headers:", ehdr.e_shoff);
    println!("  {:<35} {:#x}", "Flags:", ehdr.e_flags);
    println!("  {:<35} {} (bytes)", "Size of this header:", ehdr.e_ehsize);
    println!("  {:<35} {} (bytes)", "Size of program headers:", ehdr.e_phentsize);
    println!("  {:<35} {}", "Number of program headers:", ehdr.e_phnum);
    println!("  {:<35} {} (bytes)", "Size of section headers:", ehdr.e_shentsize);
    println!("  {:<35} {}", "Number of section headers:", ehdr.e_shnum);
    println!("  {:<35} {}", "Section header string table index:", ehdr.e_shstrndx);
}
