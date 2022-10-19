use crate::opcodes::opcodes::*;
use crate::cpu::Cpu;

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn asl(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16a = (a as u16);
        let r = u16a << 1;
        let r8 = (r & 0xFF) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if r > 0xFF {1} else {0};
    }

    let mut result : u16 = 0;
    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Accumulator => {
            operand = cpu.ac;
            set_flags(operand, cpu);
        },
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
    result = (operand as u16) << 1;
    cpu.ac = (result & 0xFF) as i8;
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn lsr(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16a = (a as u16);
        let r = u16a >> 1;
        let r8 = (r & 0xFF) as i8;
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if r > 0xFF {1} else {0};
    }

    let mut result : u16 = 0;
    let mut operand : i8 = -1;

    match opcode.address_mode {
        AddressMode::Accumulator => {
            operand = cpu.ac;
            set_flags(operand, cpu);
        },
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
    result = (operand as u16) >> 1;
    cpu.ac = (result & 0xFF) as i8;
    1
}