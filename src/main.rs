use std::env;
use std::fs;

#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

type ElfHalf = u16;
type ElfWord = u32;
type ElfXword = u64;
type ElfAddr = u64;
type ElfOff = u64;
type ElfIdent = u128;

#[derive(Debug)]
struct ElfFile {
    data: Vec<u8>,
    header: ElfHeader,
    section_headers: Vec<ElfSectionHeader>,
    program_headers: Vec<ElfProgramHeader>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ElfSectionHeader {
    name: ElfWord,
    section_type: ElfWord,
    flags: ElfXword,
    addr: ElfAddr,
    offset: ElfOff,
    size: ElfXword,
    link: ElfWord,
    info: ElfWord,
    alignment: ElfXword,
    entry_size: ElfXword,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ElfProgramHeader {
    segment_type: ElfWord,
    flags: ElfWord,
    offset: ElfOff,
    virtual_addr: ElfAddr,
    physical_addr: ElfAddr,
    file_size: ElfXword,
    memory_size: ElfXword,
    alignment: ElfXword,
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

const SHT_NULL: u32 = 0;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const SHT_RELA: u32 = 4;
const SHT_HASH: u32 = 5;
const SHT_DYNAMIC: u32 = 6;
const SHT_NOTE: u32 = 7;
const SHT_NOBITS: u32 = 8;
const SHT_REL: u32 = 9;
const SHT_SHLIB: u32 = 10;
const SHT_DYNSYM: u32 = 11;
const SHT_INIT_ARRAY: u32 = 14;
const SHT_FINI_ARRAY: u32 = 15;
const SHT_PREINIT_ARRAY: u32 = 16;
const SHT_GROUP: u32 = 17;
const SHT_SYMTAB_SHNDX: u32 = 18;
const SHT_GNU_HASH: u32 = 0x6ffffff6;
const SHT_GNU_VERNEED: u32 = 0x6ffffffe;
const SHT_GNU_VERSYM: u32 = 0x6fffffff;

const SHF_WRITE: u64 = 1 << 0;
const SHF_ALLOC: u64 = 1 << 1;
const SHF_EXECINSTR: u64 = 1 << 2;
const SHF_MERGE: u64 = 1 << 4;
const SHF_STRINGS: u64 = 1 << 5;
const SHF_INFO_LINK: u64 = 1 << 6;
const SHF_LINK_ORDER: u64 = 1 << 7;
const SHF_OS_NONCONFORMING: u64 = 1 << 8;
const SHF_GROUP: u64 = 1 << 9;
const SHF_TLS: u64 = 1 << 10;
const SHF_COMPRESSED: u64 = 1 << 11;
const SHF_EXECLUDE: u64 = 1 << 31;

const PT_NULL: u32 = 0;
const PT_LOAD: u32 = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_NOTE: u32 = 4;
const PT_SHLIB: u32 = 5;
const PT_PHDR: u32 = 6;
const PT_TLS: u32 = 7;
const PT_GNU_EH_FRAME: u32 = 0x6474e550;
const PT_GNU_STACK: u32 = 0x6474E551;
const PT_GNU_RELRO: u32 = 0x6474E552;

const PF_X: u32 = 1 << 0;
const PF_W: u32 = 1 << 1;
const PF_R: u32 = 1 << 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: readelf <command> <file>");
        std::process::exit(1);
    }
    let command = args.get(1).unwrap();
    let filename = args.get(2).unwrap();

    let elf = read_elf(filename);

    match command.as_str() {
        "header" => show_header(&elf),
        "sheader" => show_section_headers(&elf),
        "pheader" => show_program_headers(&elf),
        _ => {}
    };
}

fn read_elf(filename: &String) -> ElfFile {
    let data = fs::read(filename).unwrap();
    let (_, body, _) = unsafe { data.align_to::<ElfHeader>() };
    let header = *&body[0];

    let mut section_headers: Vec<ElfSectionHeader> = Vec::new();
    for i in 0..header.section_header_num {
        let start_addr = header.section_header_offset as usize
            + header.section_header_size as usize * i as usize;
        let end_addr = start_addr as usize + header.section_header_size as usize;
        let (_, body, _) = unsafe { data[start_addr..end_addr].align_to::<ElfSectionHeader>() };
        let section_header = &body[0];
        section_headers.push(*section_header);
    }

    let mut program_headers: Vec<ElfProgramHeader> = Vec::new();
    for i in 0..header.program_header_num {
        let start_addr = header.program_header_offset as usize
            + header.program_header_size as usize * i as usize;
        let end_addr = start_addr as usize + header.program_header_size as usize;
        let (_, body, _) = unsafe { data[start_addr..end_addr].align_to::<ElfProgramHeader>() };
        let program_header = &body[0];
        program_headers.push(*program_header);
    }

    ElfFile {
        data,
        header,
        section_headers,
        program_headers,
    }
}

fn show_header(elf: &ElfFile) {
    let header = elf.header;
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

fn show_section_headers(elf: &ElfFile) {
    let section_headers = &elf.section_headers;

    let mut table = Table::new();

    table.set_titles(row![
        "Nr", "Name", "Type", "Address", "Offset", "Size", "EntSize", "Flags", "Link", "Info",
        "Align"
    ]);
    for (i, sh) in section_headers.iter().enumerate() {
        table.add_row(row![
            i,
            get_section_name(&elf, &sh),
            get_section_type_name(sh.section_type),
            format!("0x{:x}", sh.addr),
            format!("0x{:x}", sh.offset),
            format!("0x{:x}", sh.size),
            format!("0x{:x}", sh.entry_size),
            get_section_flags(sh.flags),
            sh.link,
            sh.info,
            sh.alignment,
        ]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}

fn get_section_name(elf: &ElfFile, sh: &ElfSectionHeader) -> String {
    let header = elf.header;
    let section_headers = &elf.section_headers;
    let shstrtab = section_headers[header.string_table_index as usize];
    let start_addr = shstrtab.offset as usize + sh.name as usize;
    elf.data[start_addr..]
        .iter()
        .take_while(|&&v| v != 0)
        .map(|&v| v as char)
        .collect()
}

fn get_section_type_name(shtype: u32) -> String {
    match shtype {
        SHT_NULL => "NULL".to_string(),
        SHT_PROGBITS => "PROGBITS".to_string(),
        SHT_SYMTAB => "SYMTAB".to_string(),
        SHT_STRTAB => "STRTAB".to_string(),
        SHT_RELA => "RELA".to_string(),
        SHT_HASH => "HASH".to_string(),
        SHT_DYNAMIC => "DYNAMIC".to_string(),
        SHT_NOTE => "NOTE".to_string(),
        SHT_NOBITS => "NOBITS".to_string(),
        SHT_REL => "REL".to_string(),
        SHT_SHLIB => "SHLIB".to_string(),
        SHT_DYNSYM => "DYNSIM".to_string(),
        SHT_INIT_ARRAY => "INIT_ARRAY".to_string(),
        SHT_FINI_ARRAY => "FINI_ARRAY".to_string(),
        SHT_PREINIT_ARRAY => "PREINIT_ARRAY".to_string(),
        SHT_GROUP => "GROUP".to_string(),
        SHT_SYMTAB_SHNDX => "SYMTAB SECTION INDICES".to_string(),
        SHT_GNU_HASH => "GNU_HASH".to_string(),
        SHT_GNU_VERNEED => "VERNEED".to_string(),
        SHT_GNU_VERSYM => "VERSYM".to_string(),
        _ => format!("{:08X}: <unknown>", shtype),
    }
}

fn get_section_flags(flags: u64) -> String {
    let mut s = String::new();
    s.push(get_flag_char(flags, SHF_WRITE, 'W'));
    s.push(get_flag_char(flags, SHF_ALLOC, 'A'));
    s.push(get_flag_char(flags, SHF_EXECINSTR, 'X'));
    s.push(get_flag_char(flags, SHF_MERGE, 'M'));
    s.push(get_flag_char(flags, SHF_STRINGS, 'S'));
    s.push(get_flag_char(flags, SHF_INFO_LINK, 'I'));
    s.push(get_flag_char(flags, SHF_LINK_ORDER, 'L'));
    s.push(get_flag_char(flags, SHF_OS_NONCONFORMING, 'O'));
    s.push(get_flag_char(flags, SHF_GROUP, 'G'));
    s.push(get_flag_char(flags, SHF_TLS, 'T'));
    s.push(get_flag_char(flags, SHF_EXECLUDE, 'E'));
    s.push(get_flag_char(flags, SHF_COMPRESSED, 'C'));
    s
}

fn get_flag_char<T: Into<u64>>(flags: T, value: T, sign: char) -> char {
    let flags = flags.into() as usize;
    let value = value.into() as usize;

    if flags & value == value {
        sign
    } else {
        ' '
    }
}

fn show_program_headers(elf: &ElfFile) {
    let program_headers = &elf.program_headers;

    let mut table = Table::new();
    table.set_titles(row![
        "Type", "Offset", "VirtAddr", "PhysAddr", "FileSiz", "MemSiz", "Flags", "Align"
    ]);
    for ph in program_headers {
        table.add_row(row![
            get_segment_type(ph.segment_type),
            format!("0x{:X}", ph.offset),
            format!("0x{:X}", ph.virtual_addr),
            format!("0x{:X}", ph.physical_addr),
            format!("0x{:X}", ph.file_size),
            format!("0x{:X}", ph.memory_size),
            get_segment_flags(ph.flags),
            format!("0x{:X}", ph.alignment),
        ]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}

fn get_segment_type(segment_type: u32) -> String {
    match segment_type {
        PT_NULL => "NULL".to_string(),
        PT_LOAD => "LOAD".to_string(),
        PT_DYNAMIC => "DYNAMIC".to_string(),
        PT_INTERP => "INTERP".to_string(),
        PT_NOTE => "NOTE".to_string(),
        PT_SHLIB => "SHLIB".to_string(),
        PT_PHDR => "PHDR".to_string(),
        PT_TLS => "TLS".to_string(),
        PT_GNU_EH_FRAME => "GNU_EH_FRAME".to_string(),
        PT_GNU_STACK => "GNU_STACK".to_string(),
        PT_GNU_RELRO => "GNU_RELRO".to_string(),
        _ => format!("<unknown>: {:X}", segment_type),
    }
}

fn get_segment_flags(flags: u32) -> String {
    let mut s = String::new();
    s.push(get_flag_char(flags, PF_R, 'R'));
    s.push(get_flag_char(flags, PF_W, 'W'));
    s.push(get_flag_char(flags, PF_X, 'E'));
    s
}
