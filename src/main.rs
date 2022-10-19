#[allow(unused_parens)]
#[allow(dead_code)]
#[allow(overflowing_literals)]

mod opcodes;
mod cpu;

use opcodes::*;
use opcodes::opcodes::*;
use opcodes::sbc::*;

use cpu::*;

#[allow(overflowing_literals)]
fn main() {
    let mut cpu = Cpu::new();

    cpu.write_vec_to_memory(vec![
        0xA2, 0x00, 0xBD, 0x00, 0x10, 0x9D, 0x00, 0x20
    ], 0x0000, NO_PAGE);
    cpu.write_vec_to_memory(vec![
        0xE8, 0xE0, 0xE8, 0xD0, 0xF5
    ], 0x0008, NO_PAGE);

    cpu.pc = 0x0000;

    while true {
        let op = cpu.consume_next_byte() as u8;
        let instruction = cpu.instructions[op as usize];
        (cpu.instructions[op as usize].execute)(&instruction, &mut cpu);
    }
}
