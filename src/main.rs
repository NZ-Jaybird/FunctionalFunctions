use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate clap;
// use clap::{Arg, App, SubCommand};
use clap::{App};

pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let target_file = matches.value_of("INPUT_FILE");
    match target_file {
        Some(filename) => {
            let program = read_file(filename);
            let mut asm = assembler::Assembler::new();
            let mut vm = vm::VM::new();
            let program = asm.assemble(&program);
            match program {
                Ok(mut p) => {
                    vm.add_bytes(&mut p);
                    vm.run();
                    std::process::exit(0);
                },
                Err(..) => {}
            }
        },
        None => {
            start_repl();
        }
    }
}

// Starts a REPL that will run until the user kills it
fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

// Attempts to read a file and return the contents. Exits if unable to read the file for any reason.
fn read_file(tmp: &str) -> String {
    let filename = Path::new(tmp);
    match File::open(Path::new(&filename)) {
      Ok(mut fh) => {
        let mut contents = String::new();
        match fh.read_to_string(&mut contents) {
          Ok(_) => {
            return contents;
          },
          Err(e) => {
            println!("There was an error reading file: {:?}", e);
            std::process::exit(1);
          }
        }
      },
      Err(e) => {
        println!("File not found: {:?}", e);
        std::process::exit(1)
      }
    }
}
