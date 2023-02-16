use std;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;

use nom::types::CompleteStr;

use crate::assembler::program_parsers::*;
use crate::assembler::SymbolTable;
use crate::vm::VM;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    println!("{:#?}", result);
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".clear" => {
                    self.vm.clear_program();
                }
                ".load_file" => {
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin.read_line(&mut tmp).expect("Unable to read line from user");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("There was an error reading from the file");
                    let program = match program(CompleteStr(&contents)) {
                        // Rusts pattern matching is pretty powerful an can even be nested
                        Ok((_remainder, program)) => {
                            program
                        },
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };
                    let symbols = SymbolTable::new();
                    self.vm.program.append(&mut program.to_bytes(&symbols));
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if parsed_program.is_ok() {
                        let (_, result) = parsed_program.unwrap();
                        let symbols = SymbolTable::new();
                        let bytecode = result.to_bytes(&symbols);
                        // TODO: Make a function to let us add bytes to the VM
                        for byte in bytecode {
                            self.vm.add_byte(byte);
                        }
                    } else {
                        let results = self.parse_hex(buffer);
                        match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                        },
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                        }
                    };
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
