mod types;
mod instructions;

use self::types::OpCode;
use self::types::Status;
use self::types::InstructionOp;
use self::types::AdressingMode;
use self::types::InstructionMap;
use self::instructions::Instruction;
use self::instructions::create_instruction_map;

use std::ops::RangeInclusive;
    
pub struct MOS6502Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    status: Status,
    memory: [u8; 0xFFFF],
    instructions: InstructionMap,
}

trait Memory {
    fn write_8(&self, adress: u16, data: u8);
    fn read_8(&self, adress: u16) -> u8;
    fn write_16(&self, adress: u16, data: u16);
    fn read_16(&self, adress: u16) -> u16;
}

const MEMORY_RANGE_RAM: RangeInclusive<u16> = 0 ..= 0x1FFF;

struct Bus {
    // connect devices (meory, ppu, etc.)
}


impl MOS6502Cpu {
    pub fn new() -> Self {
        let mut cpu = MOS6502Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            memory: [0; 0xFFFF],
            instructions: create_instruction_map()
        };

        cpu.instructions[0xA9] = Instruction::Valid {
            op_code: 0xA9,
            mnemonic: "LDA",
            cycles: 2,
            adressing_mode: &Self::adressing_mode_immediate,
            operation: &Self::load_accumulator,
        };

        cpu
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let op = program[self.pc as usize];
            self.pc += 1;
    
            let instruction = &self.instructions[op as usize];

            match instruction {
                Instruction::Invalid => panic!("Invalid instruction: 0x{:02X}", op),
                Instruction::Valid { op_code, mnemonic, cycles, operation, adressing_mode } =>  (operation)(&self)
            }
        }
    }
        
    fn load_accumulator(&self) {
        println!("LDA")
    }

    fn adressing_mode_immediate(&self) -> u8 {
        println!("immediate");
        42
    }

}