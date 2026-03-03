pub mod elf;
pub mod bits64;
pub mod bits32;
pub mod parser;
pub mod helpers;

use std::{fs, process::exit};
use clap::Parser;

fn is_valid(data: &Vec<u8>) -> bool {
    if data[elf::EI_MAG0 as usize] != elf::ELFMAG0 {
        return false;
    }
    if data[elf::EI_MAG1 as usize] != elf::ELFMAG1 {
        return false;
    }
    if data[elf::EI_MAG2 as usize] != elf::ELFMAG2 {
        return false;
    }
    if data[elf::EI_MAG3 as usize] != elf::ELFMAG3 {
        return false;
    }
    return true;
}

fn main() {
    let args = parser::Args::parse();
    let data = fs::read(args.file.clone()).expect("failed to read file");

    if is_valid(&data) {
        match data[elf::EI_CLASS as usize] {
            elf::ELFCLASS32 => {
                bits32::main32::main32(&data);
            },
            elf::ELFCLASS64 => {
                bits64::main64::main64(&data, &args);
            },
            _ => {
                eprintln!("invalid EI_CLASS");
                exit(1);
            }
        }
    }
}
