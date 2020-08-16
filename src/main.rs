use std::env;
use std::fs;

type ElfHalf = u16;
type ElfWord = u32;
type ElfAddr = u64;
type ElfOff = u64;
type ElfIdent = u128;

#[repr(C)]
#[derive(Copy, Clone)]
struct ElfHeader {
    ident: ElfIdent,
    filetype: ElfHalf,
    machine: ElfHalf,
    version: ElfWord,
    entrypoint: ElfAddr,
    program_header_offset: ElfOff,
    section_header_offset: ElfOff,
    flags: ElfWord,
    elf_header_size: ElfHalf,
    program_header_size: ElfHalf,
    program_header_num: ElfHalf,
    section_header_size: ElfHalf,
    section_header_num: ElfHalf,
    string_table_index: ElfHalf,
}

const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;

const EV_CURRENT: u8 = 1;

const ELF_CLASS_NONE: u8 = 0;
const ELF_CLASS_32: u8 = 1;
const ELF_CLASS_64: u8 = 2;

const ELF_DATA_NONE: u8 = 0;
const ELF_DATA_2_LSB: u8 = 1;
const ELF_DATA_2_MSB: u8 = 2;

const ELF_OSABI_NONE: u8 = 0;

const ET_NONE: u16 = 0;
const ET_REL: u16 = 1;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const ET_CORE: u16 = 4;

const EM_NONE: u16 = 0;
const EM_X86_64: u16 = 62;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: readelf <command> <file>");
        std::process::exit(1);
    }
    let command = args.get(1).unwrap();
    let filename = args.get(2).unwrap();

    match command.as_str() {
        "header" => show_header(filename),
        _ => {}
    };
}

fn show_header(filename: &String) {
    let header = read_header(filename);
    let ident_bytes: [u8; 16] = header.ident.to_le_bytes();

    println!("ELF Header:");

    print!(" Magic:");
    for i in 0..EI_NIDENT {
        print!(" {:02X}", ident_bytes[i]);
    }
    println!();

    println!(" Class: {}", get_class_name(ident_bytes[EI_CLASS]));
    println!(" Data: {}", get_data_encoding(ident_bytes[EI_DATA]));
    println!(
        " Version: {} {}",
        ident_bytes[EI_VERSION],
        if ident_bytes[EI_VERSION] == EV_CURRENT {
            "(current)"
        } else {
            "<unknown>"
        }
    );
    println!(" OS/ABI: {}", get_osabi_name(ident_bytes[EI_OSABI]));
    println!(" ABI Version: {}", ident_bytes[EI_ABIVERSION]);
    println!(" Type: {}", get_filetype(header.filetype));
    println!(" Machine: {}", get_machine_name(header.machine));
    println!(" Version: 0x{:X}", header.version);
    println!(" Entry point address: 0x{:X}", header.entrypoint);
    println!(
        " Start of program headers: {} (bytes into file)",
        header.program_header_offset
    );
    println!(
        " Start of section headers: {} (bytes into file)",
        header.section_header_offset
    );
    println!(" Flags: 0x{:X}", header.flags);
    println!(" Size of this header: {} (bytes)", header.elf_header_size);
    println!(
        " Size of program headers: {} (bytes)",
        header.program_header_size
    );
    println!(" Number of program headers: {}", header.program_header_num);
    println!(
        " Size of section headers: {} (bytes)",
        header.section_header_size
    );
    println!(" Number of section headers: {}", header.section_header_num);
    println!(
        " Section header string table index: {}",
        header.string_table_index
    );
}

fn get_class_name(class: u8) -> String {
    match class {
        ELF_CLASS_NONE => "none".to_string(),
        ELF_CLASS_32 => "ELF32".to_string(),
        ELF_CLASS_64 => "ELF64".to_string(),
        _ => format!("<unknown>: {:X}", class),
    }
}

fn get_data_encoding(encoding: u8) -> String {
    match encoding {
        ELF_DATA_NONE => "none".to_string(),
        ELF_DATA_2_LSB => "2's complement, little endian".to_string(),
        ELF_DATA_2_MSB => "2's complement, big endian".to_string(),
        _ => format!("<unknown>: {:X}", encoding),
    }
}

fn get_osabi_name(osabi: u8) -> String {
    match osabi {
        ELF_OSABI_NONE => "UNIX - System V".to_string(),
        // (.. snip ..)
        _ => format!("<unknown>: {:X}", osabi),
    }
}

fn get_filetype(filetype: u16) -> String {
    match filetype {
        ET_NONE => "NONE (None)".to_string(),
        ET_REL => "REL (Relocatable file)".to_string(),
        ET_EXEC => "EXEC (Executable file)".to_string(),
        ET_DYN => "DYN (hared object file)".to_string(),
        ET_CORE => "CORE (Core file)".to_string(),
        _ => format!("<unknown>: {:X}", filetype),
    }
}

fn get_machine_name(machine: u16) -> String {
    match machine {
        EM_NONE => "None".to_string(),
        EM_X86_64 => "Advanced Micro Devices X86-64".to_string(),
        _ => format!("<unknown>: {:X}", machine),
    }
}

fn read_header(filename: &String) -> ElfHeader {
    let data = fs::read(filename).unwrap();
    let (_, body, _) = unsafe { data.align_to::<ElfHeader>() };
    let header = &body[0];
    *header
}
