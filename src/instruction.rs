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
  INC,
  DEC,
  ALOC
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      0 => return Opcode::LOAD,
      1 => return Opcode::ADD,
      2 => return Opcode::SUB,
      3 => return Opcode::MUL,
      4 => return Opcode::DIV,
      5 => return Opcode::HLT,
      6 => return Opcode::JMP,
      7 => return Opcode::JMPF,
      8 => return Opcode::JMPB,
      9 => return Opcode::IGL,
      10 => return Opcode::EQ,
      11 => return Opcode::NEQ,
      12 => return Opcode::GT,
      13 => return Opcode::LT,
      14 => return Opcode::GTQ,
      15 => return Opcode::LTQ,
      16 => return Opcode::JEQ,
      17 => return Opcode::INC,
      18 => return Opcode::DEC,
      19 => return Opcode::ALOC,
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
      CompleteStr("aloc") => Opcode::ALOC,
      CompleteStr("inc") => Opcode::INC,
      CompleteStr("dec") => Opcode::DEC,
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

    #[test]
    fn test_str_to_opcode_numeric() {
        let opcode = Opcode::from(CompleteStr("inc"));
        assert_eq!(opcode as u8, 17);
        let opcode = Opcode::from(CompleteStr("aloc"));
        assert_eq!(opcode as u8, 19);
    }
}
