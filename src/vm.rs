use crate::instruction::Opcode;
use crate::assembler::PIE_HEADER_LENGTH;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            heap: vec![],
            pc: PIE_HEADER_LENGTH,
            remainder: 0,
            equal_flag: false,
        }
    }

    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    pub fn add_bytes(&mut self, program: &mut Vec<u8>) {
        self.program.append(program)
    }

    pub fn clear_program(&mut self) {
        self.program.clear();
    }

    fn execute_instruction(&mut self) -> bool {
        println!("Executing instruction at {}. Program length: {}", self.pc, self.program.len());
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            },
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 >= register2;
                self.next_8_bits();
            },
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 < register2;
                self.next_8_bits();
            },
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 > register2;
                self.next_8_bits();
            },
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 != register2;
                self.next_8_bits();
            },
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            },
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            },
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::INC => {
                self.registers[self.next_8_bits() as usize] += 1;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::DEC => {
                self.registers[self.next_8_bits() as usize] -= 1;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                println!("Register: {}", register);
                let number = self.next_16_bits() as u32;
                println!("Number: {}", number);
                self.registers[register] = number as i32;
            },
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            Opcode::IGL => {
                println!("Illegal instruction encountered");
                return true;
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        println!("opcode ({:?}): {:?}", self.program[self.pc], opcode);
        self.pc += 1;
        return opcode;
    }
    
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }

    // fn verify_header(&self) -> bool {
    //     if self.program[0..4] != PIE_HEADER_PREFIX {
    //         return false;
    //     }
    //     true
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::PIE_HEADER_PREFIX;

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

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
      let mut test_vm = VM::new();
      let test_bytes = vec![5,0,0,0];
      test_vm.program = prepend_header(test_bytes);
      test_vm.run();
      assert_eq!(test_vm.pc, PIE_HEADER_LENGTH + 1);
    }

    #[test]
    fn test_opcode_igl() {
      let mut test_vm = VM::new();
      let test_bytes = vec![200,0,0,0];
      test_vm.program = prepend_header(test_bytes);
      test_vm.run();
      assert_eq!(test_vm.pc, PIE_HEADER_LENGTH + 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, PIE_HEADER_LENGTH + 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, PIE_HEADER_LENGTH + 1);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![15, 0, 1, 0, 15, 0, 1, 0, 15, 0, 1, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = (PIE_HEADER_LENGTH + 7) as i32;
        test_vm.equal_flag = true;
        test_vm.program = vec![16, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, PIE_HEADER_LENGTH + 7);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![19, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_inc_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![17, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1025);
    }

    #[test]
    fn test_dec_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![18, 0, 0, 0];
        test_vm.program = prepend_header(test_vm.program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1023);
    }
}
