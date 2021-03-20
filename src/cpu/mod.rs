mod types;
mod instructions;

use self::types::OpCode;
use self::types::Status;
use self::types::NoArgumentInstructionOp;
use self::types::SingleArgumentInstructionOp;
use self::types::AddressingMode;
use self::types::InstructionMap;
use self::types::AddressLookupResult;
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
    fn write(&mut self, address: u16, data: u8);
    fn read(&mut self, address: u16) -> u8;
}

const MEMORY_RANGE_RAM: RangeInclusive<u16> = 0 ..= 0x1FFF;

struct Bus {
    memory: [u8; 0xFFFF]
    // connect devices (meory, ppu, etc.)
}

impl Bus {
    fn new() -> Self {
        Bus {
            memory: [0; 0xFFFF]
        }
    }
}

impl Memory for Bus {
    fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    fn read(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
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
                    let (argument, pages_crossed) = adressing_mode(self);
                    (operation)(self, argument)
                }
            }
        }
    }
        
    // Load/Store Operations

    fn load_accumulator(&mut self, argument: u8) {
        self.a = argument;
        println!("LDA")
    }

    // Logical Operations

    fn and(&mut self, argument: u8) {
        self.a &= argument
    }

    // Addressing Modes

    //# region
    fn addressing_mode_immediate(&mut self) -> AddressLookupResult {
        let argument = self.memory[self.pc as usize];
        self.pc += 1;

        (argument, false)
    }
    //# endregion

    fn addressing_mode_zero_page(&mut self) -> AddressLookupResult {
        let address = self.memory[self.pc as usize];
        let argument = self.memory[address as usize];
        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_zero_page_x(&mut self) -> AddressLookupResult {
        let address = self.memory[self.pc as usize];
        let argument = self.memory[address.wrapping_add(self.x) as usize];

        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_zero_page_y(&mut self) -> AddressLookupResult {
        let address = self.memory[self.pc as usize];
        let argument = self.memory[address.wrapping_add(self.y) as usize];

        self.pc += 1;

        (argument, false)
    }

    fn addressing_mode_absolute(&mut self) -> AddressLookupResult {
        let low_byte = self.memory[self.pc as usize];
        let high_byte = self.memory[(self.pc + 1) as usize];

        self.pc += 2;

        let address = ((high_byte as u16) << 8) | low_byte as u16;
        let argument = self.memory[address as usize];

        (argument, false)
    }

    fn addressing_mode_absolute_x(&mut self) -> AddressLookupResult {
        let low_byte = self.memory[self.pc as usize];
        let high_byte = self.memory[(self.pc + 1) as usize];

        self.pc += 2;

        let address = (((high_byte as u16) << 8) | low_byte as u16) + self.x as u16;
        let argument = self.memory[address as usize];
        let pages_crossed = high_byte != (address >> 8) as u8;

        (argument, pages_crossed)
    }

    fn addressing_mode_absolute_y(&mut self) -> AddressLookupResult {
        let low_byte = self.memory[self.pc as usize];
        let high_byte = self.memory[(self.pc + 1) as usize];

        self.pc += 2;

        let address = (((high_byte as u16) << 8) | low_byte as u16) + self.y as u16;
        let argument = self.memory[address as usize];
        let pages_crossed = high_byte != (address >> 8) as u8;

        (argument, pages_crossed)
    }

    fn addressing_mode_indirect(&mut self) -> AddressLookupResult {
        let pointer_low_byte = self.memory[self.pc as usize];
        let pointer_high_byte = self.memory[(self.pc + 1) as usize];

        self.pc += 2;

        let pointer = (((pointer_high_byte as u16) << 8) | pointer_low_byte as u16) as usize;

        let low_byte = self.memory[pointer];

        let high_byte =  if pointer_low_byte == 0xFF {
            // Simulate CPU Bug: if the low byte is 0xFF, the carry bit is not propagated into the high byte when adding +1 to the pointer
            self.memory[pointer & 0xFF00]
        } else {            
            self.memory[pointer + 1]           
        };

        let address = ((high_byte as u16) << 8) | low_byte as u16;

        let argument = self.memory[address as usize];

        (argument, false)
    }

    fn addressing_mode_indirect_x(&mut self) -> AddressLookupResult {
        let zero_page_pointer = self.memory[self.pc as usize].wrapping_add(self.x);

        // Both bytes must be located in page zero
        let low_byte = self.memory[zero_page_pointer as usize];
        let high_byte = self.memory[zero_page_pointer.wrapping_add(1)  as usize];

        self.pc += 1;

        let address = ((high_byte as u16) << 8)  | low_byte as u16;

        let argument = self.memory[address as usize];
        (argument, false)
    }

    fn addressing_mode_indirect_y(&mut self) -> AddressLookupResult {
        let zero_page_pointer = self.memory[self.pc as usize] as usize;
        let low_byte = self.memory[zero_page_pointer];
        let high_byte = self.memory[zero_page_pointer.wrapping_add(1)];

        self.pc += 1;

        let address = (((high_byte as u16) << 8) | low_byte as u16) + self.y as u16;
        let argument = self.memory[address as usize];
        let pages_crossed = high_byte != (address >> 8) as u8;

        (argument, pages_crossed)
    }

    fn addressing_mode_relative(&mut self) -> AddressLookupResult {
        // cast to i8 since offset can be negative
        let relative_pc_offset = self.memory[self.pc as usize] as i8;
        
        // increment program counter before adding the offset
        self.pc += 1;

        let address = ((self.pc as i32) + (relative_pc_offset as i32)) as usize;
        let argument = self.memory[address as usize];
        
        (argument, false)
    }
}