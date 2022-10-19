use crate::opcodes::opcodes::*;
use crate::cpu::Cpu;

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn lda(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        _cpu.n = if a < 0 {1} else {0};
        _cpu.z = if a == 0 {1} else {0};
    }

    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Immediate => {
            operand = immediate_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::ZeroPage => {
            operand = zero_page_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::ZeroPageX => {
            operand = zero_page_x_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::Absolute => {
            operand = absolute_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::AbsoluteX => {
            operand = absolute_x_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::AbsoluteY => {
            operand = absolute_y_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::IndirectX => {
            operand = indirect_x_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::IndirectY => {
            operand = indirect_y_fetch(cpu);
            set_flags(operand, cpu);
        },
        _ => println!("Invalid address mode")
    }
    cpu.ac = operand;
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn ldx(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        _cpu.n = if a < 0 {1} else {0};
        _cpu.z = if a == 0 {1} else {0};
    }

    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Immediate => {
            operand = immediate_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::ZeroPage => {
            operand = zero_page_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::Absolute => {
            operand = absolute_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::AbsoluteY => {
            operand = absolute_y_fetch(cpu);
            set_flags(operand, cpu);
        },
        _ => println!("Invalid address mode")
    }
    cpu.x = operand;
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn ldy(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        _cpu.n = if a < 0 {1} else {0};
        _cpu.z = if a == 0 {1} else {0};
    }

    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Immediate => {
            operand = immediate_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::ZeroPage => {
            operand = zero_page_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::ZeroPageX => {
            operand = zero_page_x_fetch(cpu);
            set_flags(operand, cpu);
        },
       AddressMode::Absolute => {
            operand = absolute_fetch(cpu);
            set_flags(operand, cpu);
        },
        AddressMode::AbsoluteX => {
            operand = absolute_x_fetch(cpu);
            set_flags(operand, cpu);
        },
        _ => println!("Invalid address mode")
    }
    cpu.y = operand;
    1
}