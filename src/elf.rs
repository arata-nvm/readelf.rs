mod common;
pub mod elf_header;
pub mod elf_symbol;
pub mod program_header;
pub mod section_header;

use crate::elf::{
    elf_header::ElfHeader,
    elf_symbol::{ElfSymbol, ElfSymbolTable},
    program_header::ElfProgramHeader,
    section_header::ElfSectionHeader,
};
use std::fs;

type ElfHalf = u16;
type ElfWord = u32;
type ElfXword = u64;
type ElfAddr = u64;
type ElfOff = u64;
type ElfSection = u16;
type ElfIdent = u128;

#[derive(Debug)]
pub struct ElfFile {
    pub data: Vec<u8>,
    pub header: ElfHeader,
    pub section_headers: Vec<ElfSectionHeader>,
    pub program_headers: Vec<ElfProgramHeader>,
    pub symbol_tables: Vec<ElfSymbolTable>,
}

impl ElfFile {
    pub fn read_from_file(filename: &String) -> Self {
        let data = fs::read(filename).unwrap();

        let header = Self::read_header(&data);
        let section_headers = Self::read_section_headers(&header, &data);
        let program_headers = Self::read_program_headers(&header, &data);
        let symbols = Self::read_symbols(&section_headers, &data);

        Self {
            data,
            header,
            section_headers,
            program_headers,
            symbol_tables: symbols,
        }
    }

    fn read_header(data: &Vec<u8>) -> ElfHeader {
        let (_, body, _) = unsafe { data.align_to::<ElfHeader>() };
        *&body[0]
    }

    fn read_section_headers(header: &ElfHeader, data: &Vec<u8>) -> Vec<ElfSectionHeader> {
        let mut section_headers: Vec<ElfSectionHeader> = Vec::new();
        for i in 0..header.section_header_num {
            let start_addr = header.section_header_offset as usize
                + header.section_header_size as usize * i as usize;
            let end_addr = start_addr as usize + header.section_header_size as usize;
            let (_, body, _) = unsafe { data[start_addr..end_addr].align_to::<ElfSectionHeader>() };
            let section_header = &body[0];
            section_headers.push(*section_header);
        }
        section_headers
    }

    fn read_program_headers(header: &ElfHeader, data: &Vec<u8>) -> Vec<ElfProgramHeader> {
        let mut program_headers: Vec<ElfProgramHeader> = Vec::new();
        for i in 0..header.program_header_num {
            let start_addr = header.program_header_offset as usize
                + header.program_header_size as usize * i as usize;
            let end_addr = start_addr as usize + header.program_header_size as usize;
            let (_, body, _) = unsafe { data[start_addr..end_addr].align_to::<ElfProgramHeader>() };
            let program_header = &body[0];
            program_headers.push(*program_header);
        }
        program_headers
    }

    fn read_symbols(
        section_headers: &Vec<ElfSectionHeader>,
        data: &Vec<u8>,
    ) -> Vec<ElfSymbolTable> {
        section_headers
            .iter()
            .enumerate()
            .filter(|(_, sh)| match sh.section_type {
                section_header::SHT_SYMTAB | section_header::SHT_DYNSYM => true,
                _ => false,
            })
            .map(|(i, sh)| ElfSymbolTable {
                index: i,
                symbols: Self::read_symbols_from_section(sh, data),
            })
            .collect()
    }

    fn read_symbols_from_section(sh: &ElfSectionHeader, data: &Vec<u8>) -> Vec<ElfSymbol> {
        let mut symbols: Vec<ElfSymbol> = Vec::new();
        let symbol_num = sh.size / sh.entry_size;
        for i in 0..symbol_num {
            let start_addr = sh.offset as usize + sh.entry_size as usize * i as usize;
            let end_addr = start_addr as usize + sh.entry_size as usize;
            let (_, body, _) = unsafe { data[start_addr..end_addr].align_to::<ElfSymbol>() };
            let symbol = &body[0];
            symbols.push(*symbol);
        }
        symbols
    }
}
