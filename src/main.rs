use std::collections::HashMap;

type OpCode = u8;
type Status = u8;
type InstructionOp = dyn Fn(&MOS6502Cpu);
type AdressingMode = dyn Fn(&MOS6502Cpu) -> u8;

struct Instruction {
    op_code: OpCode,
    mnemonic: String,
    cycles: u8,
    operation: &'static InstructionOp,
    adressing_mode: &'static AdressingMode,
}

struct MOS6502Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    status: Status,
    memory: [u8; 0xFFFF],
    instruction_map: HashMap<OpCode, Instruction>
}

impl MOS6502Cpu {
    fn new() -> Self {
        MOS6502Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            memory: [0; 0xFFFF],
            instruction_map: HashMap::new()
        }
    }


    fn register_instruction(&mut self, instruction: Instruction) {
        self.instruction_map.insert(instruction.op_code, instruction);
    }

    fn load_accumulator(&self) {
        println!("LDA")
    }

    fn adressing_mode_immediate(&self) -> u8 {
        println!("immmediate");
        42
    }

    fn run(&mut self, program: Vec<u8>) {
        self.pc = 0;

        self.register_instruction(Instruction {
            op_code: 0xA9,
            mnemonic: "LDA".to_string(),
            cycles: 2,
            adressing_mode: &Self::adressing_mode_immediate,
            operation: &Self::load_accumulator,
        });

        loop {
            let op = program[self.pc as usize];
            self.pc += 1;
    
            let instruction = self.instruction_map.get(&op).expect(format!("Unimplemented opcode: {}", op).as_str());
            (instruction.operation)(&self);
        }
    }
}

fn main() {
    let mut cpu = MOS6502Cpu::new();
    cpu.run(vec![0xA9, 0xA9]);
}
