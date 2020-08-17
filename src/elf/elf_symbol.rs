use crate::elf::*;

use prettytable::{cell, format, row, Table};

#[derive(Debug)]
pub struct ElfSymbolTable {
    pub index: usize,
    pub symbols: Vec<ElfSymbol>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ElfSymbol {
    pub name: ElfWord,
    pub info: u8,
    pub other: u8,
    pub section_index: ElfSection,
    pub value: ElfAddr,
    pub size: ElfXword,
}

#[repr(C)]
#[derive(Debug)]
pub struct ElfSymbolInfo {
    pub bound_to: ElfHalf,
    pub flags: ElfHalf,
}

const STB_LOCAL: u8 = 0;
const STB_GLOBAL: u8 = 1;
const STB_WEAK: u8 = 2;

const STT_NOTYPE: u8 = 0;
const STT_OBJECT: u8 = 1;
const STT_FUNC: u8 = 2;
const STT_SECTION: u8 = 3;
const STT_FILE: u8 = 4;
const STT_COMMON: u8 = 5;
const STT_TLS: u8 = 6;

const STV_DEFAULT: u8 = 0;
const STV_INTERNAL: u8 = 1;
const STV_HIDDEN: u8 = 2;
const STV_PROTECTED: u8 = 3;

const SHN_UNDEF: u16 = 0;
const SHN_ABS: u16 = 0xfff1;
const SHN_COMMON: u16 = 0xfff2;

impl ElfFile {
    pub fn show_symbol_tables(&self) {
        let shstrtab = self.section_headers[self.header.string_table_index as usize];
        for (i, st) in self.symbol_tables.iter().enumerate() {
            if i != 0 {
                println!();
            }
            let section_name =
                self.get_name_from_strtab(&shstrtab, self.section_headers[st.index].name as usize);
            let strtab = &self.section_headers[st.index + 1];
            self.show_symbols(&section_name, &st, strtab);
        }
    }

    fn show_symbols(&self, section_name: &String, st: &ElfSymbolTable, strtab: &ElfSectionHeader) {
        println!("Symbol tables '{}': ", section_name);

        let mut table = Table::new();
        table.set_titles(row![
            "Num", "Value", "Size", "Type", "Bind", "Vis", "Ndx", "Name",
        ]);
        for (i, symbol) in st.symbols.iter().enumerate() {
            table.add_row(row![
                format!("{}", i),
                format!("{:X}", symbol.value),
                format!("{}", symbol.size),
                self.get_symbol_type(symbol.info),
                self.get_symbol_binding(symbol.info),
                self.get_symbol_visibility(symbol.other),
                self.get_symbol_index_type(symbol.section_index),
                self.get_name_from_strtab(strtab, symbol.name as usize),
            ]);
        }
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.printstd();
    }

    fn get_symbol_binding(&self, info: u8) -> String {
        let binding = info >> 4;
        match binding {
            STB_LOCAL => "LOCAL".to_string(),
            STB_GLOBAL => "GLOBAL".to_string(),
            STB_WEAK => "WEAK".to_string(),
            _ => format!("<unknown>: {}", binding),
        }
    }

    fn get_symbol_type(&self, info: u8) -> String {
        let type_ = info & 0xf;
        match type_ {
            STT_NOTYPE => "NOTYPE".to_string(),
            STT_OBJECT => "OBJECT".to_string(),
            STT_FUNC => "FUNC".to_string(),
            STT_SECTION => "SECTION".to_string(),
            STT_FILE => "FILE".to_string(),
            STT_COMMON => "COMMON".to_string(),
            STT_TLS => "TLS".to_string(),
            _ => format!("<unknown>: {}", type_),
        }
    }

    fn get_symbol_visibility(&self, other: u8) -> String {
        let visibility = other & 0x3;
        match visibility {
            STV_DEFAULT => "DEFAULT".to_string(),
            STV_INTERNAL => "INTERNAL".to_string(),
            STV_HIDDEN => "HIDDEN".to_string(),
            STV_PROTECTED => "PROTECTED".to_string(),
            _ => "<unknown>".to_string(),
        }
    }

    fn get_symbol_index_type(&self, type_: u16) -> String {
        match type_ {
            SHN_UNDEF => "UND".to_string(),
            SHN_ABS => "ABS".to_string(),
            SHN_COMMON => "COM".to_string(),
            _ => format!("{:3}", type_),
        }
    }
}
