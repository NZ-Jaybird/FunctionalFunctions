use std;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;

use nom::types::CompleteStr;

use crate::assembler::PIE_HEADER_LENGTH;
use crate::assembler::PIE_HEADER_PREFIX;
use crate::assembler::program_parsers::*;
use crate::assembler::SymbolTable;
use crate::vm::VM;

use crate::repl::system_operations::SystemOperations;
use crate::repl::system_operations::SystemOperationsImpl;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        let mut repl_vm = VM::new();
        repl_vm.program = Self::prepend_header(vec![]);
        REPL {
            vm: repl_vm,
            command_buffer: vec![],
        }
    }

    fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
        let mut prepension = vec![];
        for byte in PIE_HEADER_PREFIX.iter() {
            prepension.push(byte.clone());
        }
        while prepension.len() < PIE_HEADER_LENGTH {
            prepension.push(0);
        }
        prepension.append(&mut b);
        prepension
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
            self.run_once(&mut SystemOperationsImpl::new());
        }
    }

    fn run_once(&mut self, system_ops: &mut impl SystemOperations) {
        // This allocates a new String in which to store whatever the user types each iteration.
        // TODO: Figure out how create this outside of the loop and re-use it every iteration
        let mut buffer = String::new();

        // Blocking call until the user types in a command
        system_ops.request_input();

        // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
        // have to do that there for the user to see our `>>> ` prompt.
        print!(">>> ");
        io::stdout().flush().expect("Unable to flush stdout");

        // Here we'll look at the string the user gave us.
        system_ops.read_line(&mut buffer);

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

                system_ops.read_line(&mut tmp);
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
                        return;
                    }
                };
                let symbols = SymbolTable::new();
                self.vm.program.append(&mut program.to_bytes(&symbols));
            }
            _ => {
                let parsed_program = program(CompleteStr(buffer));
                if parsed_program.is_ok() {
                    let (_, result) = parsed_program.unwrap();
                    println!("{:?}", result);
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

    pub fn get_register(&self, index: usize) -> i32 {
        return self.vm.registers[index];
    }
}

pub mod system_operations;

pub struct TestSystemOperations
{
    command: String
}

impl TestSystemOperations {
    pub fn new(command: &str) -> TestSystemOperations {
        TestSystemOperations {
            command: command.to_string()
        }
    }
}

impl SystemOperations for TestSystemOperations {
    fn read_line(&self, buffer: &mut String) {
        buffer.push_str(&self.command.to_string());
    }
}

#[test]
fn test_main_loop() {
    let mut repl = REPL::new();
    repl.run_once(&mut TestSystemOperations::new("load $0 #3"));
    assert_eq!(repl.get_register(0), 3);
}
