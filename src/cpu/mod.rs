mod types;
mod instructions;

use self::types::OpCode;
use self::types::Status;
use self::types::NoArgumentInstructionOp;
use self::types::SingleArgumentInstructionOp;
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
        MOS6502Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            memory: [0; 0xFFFF],
            instructions: create_instruction_map()
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let op = program[self.pc as usize];
            self.pc += 1;
    
            let instruction = self.instructions[op as usize];

            match instruction {
                Instruction::Invalid => panic!("Invalid instruction: 0x{:02X}", op),
                Instruction::NoArgument { op_code, mnemonic, cycles, operation } =>  {
                    (operation)(self)
                },
                Instruction::SingleArgument { op_code, mnemonic, cycles, operation, adressing_mode } =>  {
                    let argument = adressing_mode(self);
                    (operation)(self, argument)
                }
            }
        }
    }
        
    fn load_accumulator(&mut self, argument: u8) {
        // self.a = 
        println!("LDA")
    }

    fn adressing_mode_immediate(&mut self) -> u8 {
        let argument = self.memory[self.pc as usize];
        
        self.pc += 1;
        argument
    }

    fn adressing_mode_zero_page(&mut self) -> u8 {
        let adress = self.memory[self.pc as usize];
        let argument = self.memory[adress as usize];

        self.pc += 1;
        argument
    }

    fn adressing_mode_zero_page_x(&mut self) -> u8 {
        let adress = self.memory[self.pc as usize];
        let argument = self.memory[adress.wrapping_add(self.x) as usize];
        self.pc += 1;
        argument
    }
}