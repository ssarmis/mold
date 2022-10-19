use crate::opcodes::opcodes::*;
use crate::cpu::*;

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn jmp(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    let mut result : u16 = 0;
    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Absolute => {
            let operand = cpu.consume_next_word();
            
            let half0 = operand & 0x00FF;
            let half1 = operand & 0xFF00;

            cpu.pc = (half0) | (half1 >> 8);
        },
        AddressMode::Indirect => {
            let operand = cpu.consume_next_byte();
            let a = cpu.read_word_from_memory(operand as u16, ZERO_PAGE);

            let half0 = a & 0x00FF;
            let half1 = a & 0xFF00;

            cpu.pc = (half0) | (half1 >> 8);
        },
        _ => println!("Invalid address mode")
    }
    1
}

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn jsr(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    let mut result : u16 = 0;
    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Absolute => {
            let operand = cpu.consume_next_word();
            
            let half0 = operand & 0x00FF;
            let half1 = operand & 0xFF00;

            cpu.push_to_stack((((cpu.pc + 2) & 0xFF00) >> 8) as i8);
            cpu.push_to_stack(((cpu.pc + 2) & 0x00FF) as i8);
            cpu.pc = (half0) | (half1 >> 8);
        },
        _ => println!("Invalid address mode")
    }
    1
}