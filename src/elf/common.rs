use crate::elf::{section_header::ElfSectionHeader, ElfFile};

pub fn get_flag_char<T: Into<u64>>(flags: T, value: T, sign: char) -> char {
    let flags = flags.into() as usize;
    let value = value.into() as usize;

    if flags & value == value {
        sign
    } else {
        ' '
    }
}

impl ElfFile {
    pub fn get_name_from_strtab(&self, sh: &ElfSectionHeader, index: usize) -> String {
        let start_addr = sh.offset as usize + index;
        self.data[start_addr..]
            .iter()
            .take_while(|&&v| v != 0)
            .map(|&v| v as char)
            .collect()
    }
}
