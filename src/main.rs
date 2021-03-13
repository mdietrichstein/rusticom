use rusticom::cpu::MOS6502Cpu;

fn main() {
    let mut cpu = MOS6502Cpu::new();
    cpu.run(vec![0xA9, 0xA9]);
}
