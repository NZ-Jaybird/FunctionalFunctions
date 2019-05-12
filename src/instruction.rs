#[derive(PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    IGL

  impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MUL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            _ => return Opcode::IGL
        }
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: Opcode
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction {
      opcode: opcode
    }
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
}
