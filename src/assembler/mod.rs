use crate::assembler::program_parsers::program;
use crate::assembler::program_parsers::Program;
use crate::instruction::Opcode;

use nom::types::CompleteStr;

pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new()
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                let mut assembled_program = self.write_pie_header();
                self.process_first_phase(&program);
                let mut body = self.process_second_phase(&program);

                assembled_program.append(&mut body);
                Some(assembled_program)
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }
    
    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }
    
    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbols);
            program.append(&mut bytes);
        }
        program
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                match i.label_name() {
                    Some(name) => {
                        let symbol = Symbol::new(name, /*SymbolType::Label,*/ c);
                        self.symbols.add_symbol(symbol);
                    },
                    None => {}
                };
            }
            c += 4;
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in PIE_HEADER_PREFIX {
            header.push(byte.clone());
        }
        while header.len() < PIE_HEADER_LENGTH {
            header.push(0 as u8);
        }
        header
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    Number{value: i32},
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { literal: String }
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u8,
    // symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, /*symbol_type: SymbolType,*/ offset: u8) -> Symbol {
        Symbol{
            name,
            // symbol_type,
            offset
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable{
            symbols: vec![]
        }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> u8 {
        for symbol in &self.symbols {
            if symbol.name == s {
                return symbol.offset;
            }
        }
        println!("Symbol {} not found", s);
        std::process::exit(1);
    }
}

pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;
pub mod instruction_parsers;
pub mod program_parsers;
pub mod directive_parsers;
pub mod label_parsers;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vm::VM;
    
    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), /*SymbolType::Label,*/ 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(v, 12);
    }

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = "load $0 #100\nload $0 #100\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
        let mut program = asm.assemble(test_string).unwrap();
        println!("{:?}", program);
        let mut vm = VM::new();
        assert_eq!(program.len(), PIE_HEADER_LENGTH + 28);
        vm.add_bytes(&mut program);
        assert_eq!(vm.program.len(), PIE_HEADER_LENGTH + 28);
    }
}