use crate::elf::common::get_flag_char;
use crate::elf::*;

use prettytable::{cell, format, row, Table};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ElfProgramHeader {
    pub segment_type: ElfWord,
    pub flags: ElfWord,
    pub offset: ElfOff,
    pub virtual_addr: ElfAddr,
    pub physical_addr: ElfAddr,
    pub file_size: ElfXword,
    pub memory_size: ElfXword,
    pub alignment: ElfXword,
}

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

impl ElfFile {
    pub fn show_program_headers(&self) {
        let program_headers = &self.program_headers;

        let mut table = Table::new();
        table.set_titles(row![
            "Type", "Offset", "VirtAddr", "PhysAddr", "FileSiz", "MemSiz", "Flags", "Align"
        ]);
        for ph in program_headers {
            table.add_row(row![
                self.get_segment_type(ph.segment_type),
                format!("0x{:X}", ph.offset),
                format!("0x{:X}", ph.virtual_addr),
                format!("0x{:X}", ph.physical_addr),
                format!("0x{:X}", ph.file_size),
                format!("0x{:X}", ph.memory_size),
                self.get_segment_flags(ph.flags),
                format!("0x{:X}", ph.alignment),
            ]);
        }

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.printstd();
    }

    fn get_segment_type(&self, segment_type: u32) -> String {
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

    fn get_segment_flags(&self, flags: u32) -> String {
        let mut s = String::new();
        s.push(get_flag_char(flags, PF_R, 'R'));
        s.push(get_flag_char(flags, PF_W, 'W'));
        s.push(get_flag_char(flags, PF_X, 'E'));
        s
    }
}
