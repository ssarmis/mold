use crate::opcodes::opcodes::*;
use crate::cpu::Cpu;

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn cmp(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16ac : u16 = _cpu.ac as u16 & 0x00ff;
        let u16a  : u16 = a as u16 & 0x00ff;

        let r : i16 = (u16ac - u16a) as i16;
        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if r > 0xff {1} else {0};
    }

    let mut result : u16 = 0;
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
    1
}

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn cpx(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16x : i16 = (_cpu.x as u16 & 0x00ff) as i16;
        let u16a  : i16 = (a as u16 & 0x00ff) as i16;

        let r : i16 = u16x - u16a;
        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if r > 0xff {1} else {0};
    }

    let mut result : u16 = 0;
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
        _ => println!("Invalid address mode")
    }
    1
}

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn cpy(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16y : u16 = _cpu.y as u16 & 0x00ff;
        let u16a  : u16 = a as u16 & 0x00ff;

        let r : i16 = (u16y - u16a) as i16;
        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if r > 0xff {1} else {0};
    }

    let mut result : u16 = 0;
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
        _ => println!("Invalid address mode")
    }
    1
}