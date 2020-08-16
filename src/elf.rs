mod common;
pub mod elf_header;
pub mod program_header;
pub mod section_header;

use crate::elf::{
    elf_header::ElfHeader, program_header::ElfProgramHeader, section_header::ElfSectionHeader,
};
use std::fs;

type ElfHalf = u16;
type ElfWord = u32;
type ElfXword = u64;
type ElfAddr = u64;
type ElfOff = u64;
type ElfIdent = u128;

#[derive(Debug)]
pub struct ElfFile {
    pub data: Vec<u8>,
    pub header: ElfHeader,
    pub section_headers: Vec<ElfSectionHeader>,
    pub program_headers: Vec<ElfProgramHeader>,
}

impl ElfFile {
    pub fn read_from_file(filename: &String) -> Self {
        let data = fs::read(filename).unwrap();

        let header = Self::read_header(&data);
        let section_headers = Self::read_section_headers(&header, &data);
        let program_headers = Self::read_program_headers(&header, &data);

        Self {
            data,
            header,
            section_headers,
            program_headers,
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
}
