use crate::assembler::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::operand_parsers::operand;
use crate::assembler::directive_parsers::directive;
use crate::assembler::label_parsers::label_declaration;
use crate::assembler::SymbolTable;
use nom::types::CompleteStr;
use nom::multispace;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

// Handles instructions of the following form:
// LOAD $0 #100
named!(instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                directive: None,
                label: None,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);

named!(instruction_two<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                directive: None,
                label: None,
                operand1: None,
                operand2: None,
                operand3: None,
            }
        )
    )
);

named!(instruction_three<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        r3: register >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                directive: None,
                label: None,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3)
            }
        )
    )
);

// Will try to parse out any of the Instruction forms
named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined |
            directive
        ) >>
        (
            ins
        )
    )
);

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(Token::Op { code }) => {
                results.push(code as u8);
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, symbols, &mut results),
                None => {}
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        return results;
    }

    pub fn is_label(&self) -> bool {
        self.label != None
    }

    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(Token::LabelDeclaration { name }) => {
                return Some(name.clone());
            },
            _ => {
                return None
            }
        };
    }

    fn extract_operand(t: &Token, symbols: &SymbolTable, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::Number { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            Token::LabelUsage { name } => {
                let offset = symbols.symbol_value(name);
                results.push(offset);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    directive: None,
                    label: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Number { value: 100 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    directive: None,
                    label: None,
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_three() {
        let result = instruction_three(CompleteStr("add $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    directive: None,
                    label: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 })
                }
            ))
        );
    }
}
