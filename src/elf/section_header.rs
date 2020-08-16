use crate::elf::common::get_flag_char;
use crate::elf::*;

use prettytable::{cell, format, row, Table};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ElfSectionHeader {
    pub name: ElfWord,
    pub section_type: ElfWord,
    pub flags: ElfXword,
    pub addr: ElfAddr,
    pub offset: ElfOff,
    pub size: ElfXword,
    pub link: ElfWord,
    pub info: ElfWord,
    pub alignment: ElfXword,
    pub entry_size: ElfXword,
}

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

impl ElfFile {
    pub fn show_section_headers(&self) {
        let section_headers = &self.section_headers;

        let mut table = Table::new();

        table.set_titles(row![
            "Nr", "Name", "Type", "Address", "Offset", "Size", "EntSize", "Flags", "Link", "Info",
            "Align"
        ]);
        for (i, sh) in section_headers.iter().enumerate() {
            table.add_row(row![
                i,
                self.get_section_name(&sh),
                self.get_section_type_name(sh.section_type),
                format!("0x{:x}", sh.addr),
                format!("0x{:x}", sh.offset),
                format!("0x{:x}", sh.size),
                format!("0x{:x}", sh.entry_size),
                self.get_section_flags(sh.flags),
                sh.link,
                sh.info,
                sh.alignment,
            ]);
        }

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.printstd();
    }

    fn get_section_name(&self, sh: &ElfSectionHeader) -> String {
        let header = &self.header;
        let section_headers = &self.section_headers;
        let shstrtab = &section_headers[header.string_table_index as usize];
        let start_addr = shstrtab.offset as usize + sh.name as usize;
        self.data[start_addr..]
            .iter()
            .take_while(|&&v| v != 0)
            .map(|&v| v as char)
            .collect()
    }

    fn get_section_type_name(&self, shtype: u32) -> String {
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

    fn get_section_flags(&self, flags: u64) -> String {
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
}
