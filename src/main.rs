use readelf::elf::ElfFile;
use std::env;

extern crate prettytable;
extern crate readelf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: readelf <command> <file>");
        std::process::exit(1);
    }
    let command = args.get(1).unwrap();
    let filename = args.get(2).unwrap();

    let elf = ElfFile::read_from_file(filename);

    match command.as_str() {
        "all" => {
            elf.show_header();
            println!();
            elf.show_section_headers();
            println!();
            elf.show_program_headers();
            println!();
            elf.show_symbol_tables();
        }
        "header" => elf.show_header(),
        "sheader" => elf.show_section_headers(),
        "pheader" => elf.show_program_headers(),
        "symbol" => elf.show_symbol_tables(),
        _ => {}
    };
}
