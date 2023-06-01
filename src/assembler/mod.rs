use crate::assembler::instruction_parsers::AssemblerInstruction;
use crate::assembler::program_parsers::program;
use crate::assembler::program_parsers::Program;
use crate::instruction::Opcode;

use nom::types::CompleteStr;

pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug, PartialEq)]
pub enum AssemblerSection {
    Code,
    Unknown
}

impl From<&str> for AssemblerSection {
    fn from(name: &str) -> Self { 
        match name {
            "code" => { AssemblerSection::Code },
            _ => todo!("{:?}", name)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AssemblerError {
    UnknownDirectiveFound{directive: String},
    NoSegmentDeclarationFound{instruction: u32},
    SymbolAlreadyDeclared,
    StringConstantDeclaredWithoutLabel{instruction: u32},
    ParseError{error: String}
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub ro: Vec<u8>,
    pub bytecode: Vec<u8>,
    ro_offset: u8,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            current_section: None,
            current_instruction: 0,
            errors: vec![]
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                let mut assembled_program = self.write_pie_header();
                self.process_first_phase(&program);

                if !self.errors.is_empty() {
                    // TODO: Can we avoid a clone here?
                    return Err(self.errors.clone());
                };

                let mut body = self.process_second_phase(&program);

                assembled_program.append(&mut body);
                Ok(assembled_program)
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                Err(vec![AssemblerError::ParseError{ error: e.to_string() }])
            }
        }
    }
    
    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels_and_directives(p);
        self.phase = AssemblerPhase::Second;
    }
    
    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        self.current_instruction = 0;
        let mut program = vec![];
        for i in &p.instructions {
            if i.is_opcode() {
                // Opcodes know how to properly transform themselves into 32-bits, so we can just call `to_bytes` and append to our program
                let mut bytes = i.to_bytes(&self.symbols);
                program.append(&mut bytes);
            }
            if i.is_directive() {
                // In this phase, we can have directives but of different types than we care about in the first pass. The Directive itself can check which pass the Assembler
                // is in and decide what to do about it
                self.process_directive(i);
            }
            self.current_instruction += 1
        }
        program
    }

    fn extract_labels_and_directives(&mut self, p: &Program) {
        for i in &p.instructions {
            if i.is_label() {
                if self.current_section.is_some() {
                    let name: String = match i.label_name() {
                        Some(name) => { name },
                        None => {
                            self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel{instruction: self.current_instruction});
                            return;
                        }
                    };
                    if self.symbols.has_symbol(&name) {
                        self.errors.push(AssemblerError::SymbolAlreadyDeclared);
                        return;
                    }
                    let symbol = Symbol::new(name.to_string(), SymbolType::Label);
                    self.symbols.add_symbol(symbol);
                } else {
                    // If we have *not* hit a segment header yet, then we have a label outside of a segment, which is not allowed
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound{instruction: self.current_instruction});
                }
            }
            if i.is_directive() {
                self.process_directive(i);
            }
            self.current_instruction += 1;
        }
    }

    fn process_directive(&mut self, i: &AssemblerInstruction) { 
        // First let’s make sure we have a parseable name 
        let directive_name = match i.get_directive_name() { 
            Some(name) => { name }, 
            None => { 
                println!("Directive has an invalid name: {:?}", i); 
                return; 
            } 
        };

        // Now check if there were any operands.
        if i.has_operands() {
            // If it _does_ have operands, we need to figure out which directive it was
            match directive_name.as_ref() {
                // If this is the operand, we're declaring a null terminated string
                "asciiz" => {
                    self.handle_asciiz(i);
                }
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound{ directive: directive_name.clone() });
                    return;
                }
            }
        } else {
            // If there were not any operands, (e.g., `.code`), then we know it is a section header
            self.process_section_header(&directive_name);
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();
        // Only specific section names are allowed
        if new_section == AssemblerSection::Unknown {
            println!("Found an section header that is unknown: {:#?}", header_name);
            return;
        }
        self.current_section = Some(new_section);
    }

    fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
        // Being a constant declaration, this is only meaningful in the first pass
        if self.phase != AssemblerPhase::First { return; }
    
        // In this case, operand1 will have the entire string we need to read in to RO memory
        match i.get_string_constant() {
            Some(s) => {
                match i.label_name() {
                    Some(name) => { self.symbols.set_symbol_offset(name, self.ro_offset); }
                    None => {
                        // This would be someone typing:
                        // .asciiz 'Hello'
                        println!("Found a string constant with no associated label!");
                        return;
                    }
                };
                // We'll read the string into the read-only section byte-by-byte
                for byte in s.as_bytes() {
                    self.ro.push(*byte);
                    self.ro_offset += 1;
                }
                // This is the null termination bit we are using to indicate a string has ended
                self.ro.push(0);
                self.ro_offset += 1;
            }
            None => {
                // This just means someone typed `.asciiz` for some reason
                println!("String constant following an .asciiz was empty");
            }
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
    symbol_type: SymbolType,
    offset: u8
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Symbol {
        Symbol{
            name,
            symbol_type,
            offset: 0
        }
    }

    pub fn set_offset(&mut self, offset: u8) {
        self.offset = offset;
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

    pub fn has_symbol(&mut self, name: &String) -> bool {
        match self.symbols.iter().find(|&s| s.name == *name) {
            Some(..) => { true },
            None => { false }
        }
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

    pub fn set_symbol_offset(&mut self, name: String, offset: u8) {
        let position: Option<usize> = self.symbols.iter().position(|s| s.name == name);
        match position {
            Some(index) => {
                self.symbols[index].set_offset(offset)
            },
            None => {}
        }
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
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label);
        sym.add_symbol(new_symbol);
        sym.set_symbol_offset("test".to_string(), 12);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(v, 12);
    }

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = ".code\nload $0 #100\nload $0 #100\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
        let mut program = asm.assemble(test_string).unwrap();
        println!("{:?}", program);
        let mut vm = VM::new();
        assert_eq!(program.len(), PIE_HEADER_LENGTH + 28);
        vm.add_bytes(&mut program);
        assert_eq!(vm.program.len(), PIE_HEADER_LENGTH + 28);
    }
}