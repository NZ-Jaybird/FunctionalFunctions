use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
  LOAD,
  ADD,
  SUB,
  MUL,
  DIV,
  HLT,
  JMP,
  JMPF, // 7
  JMPB,
  IGL,
  EQ, // 9
  NEQ,
  GT,
  LT,
  GTQ,
  LTQ,
  JEQ,
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      5 => return Opcode::HLT,
      0 => return Opcode::LOAD,
      1 => return Opcode::ADD,
      2 => return Opcode::SUB,
      3 => return Opcode::MUL,
      4 => return Opcode::DIV,
      6 => return Opcode::JMP,
      7 => return Opcode::JMPF,
      8 => return Opcode::JMPB,
      9 => return Opcode::EQ,
      10 => return Opcode::NEQ,
      11 => return Opcode::GT,
      12 => return Opcode::LT,
      13 => return Opcode::GTQ,
      14 => return Opcode::LTQ,
      15 => return Opcode::JEQ,
      _ => return Opcode::IGL,
    }
  }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
  fn from(v: CompleteStr<'a>) -> Self {
    match v {
      CompleteStr("load") => Opcode::LOAD,
      CompleteStr("add") => Opcode::ADD,
      CompleteStr("sub") => Opcode::SUB,
      CompleteStr("mul") => Opcode::MUL,
      CompleteStr("div") => Opcode::DIV,
      CompleteStr("hlt") => Opcode::HLT,
      CompleteStr("jmp") => Opcode::JMP,
      CompleteStr("jmpf") => Opcode::JMPF,
      CompleteStr("jmpb") => Opcode::JMPB,
      CompleteStr("eq") => Opcode::EQ,
      CompleteStr("neq") => Opcode::NEQ,
      CompleteStr("gte") => Opcode::GTQ,
      CompleteStr("gt") => Opcode::GT,
      CompleteStr("lte") => Opcode::LTQ,
      CompleteStr("lt") => Opcode::LT,
      CompleteStr("jmpe") => Opcode::JEQ,
      _ => Opcode::IGL,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: Opcode,
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction { opcode: opcode }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
    let opcode = Opcode::HLT;
    assert_eq!(opcode, Opcode::HLT);
  }

  #[test]
  fn test_create_instruction() {
    let instruction = Instruction::new(Opcode::HLT);
    assert_eq!(instruction.opcode, Opcode::HLT);
  }

  #[test]
fn test_str_to_opcode() {
    let opcode = Opcode::from(CompleteStr("load"));
    assert_eq!(opcode, Opcode::LOAD);
    let opcode = Opcode::from(CompleteStr("illegal"));
    assert_eq!(opcode, Opcode::IGL);
}
}
