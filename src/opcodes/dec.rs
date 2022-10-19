use crate::opcodes::opcodes::*;
use crate::cpu::*;

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn dec(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16a : u16 = a as u16 & 0x00ff;

        let r : i16 = u16a as i16 - 1;
        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
    }

    let mut a : i8 = 0;
    let mut p : u16 = NO_PAGE;
    let mut add : u16 = 0;

    match opcode.address_mode {
        AddressMode::ZeroPage => {
            let operand = cpu.consume_next_byte();
            add = operand as u16;
            a = cpu.read_from_memory(operand as u16, ZERO_PAGE);
            set_flags(a, cpu);
            p = ZERO_PAGE;
        },
        AddressMode::ZeroPageX => {
            let operand = cpu.consume_next_byte() + cpu.x;
            add = operand as u16;
            a = cpu.read_from_memory(operand as u16, ZERO_PAGE);
            set_flags(a, cpu);
            p = ZERO_PAGE;
        },
        AddressMode::Absolute => {
            let operand = cpu.consume_next_word();
            let swapped_operand : u16 = (((operand) & 0xff) << 8) | (((operand) & 0xff00) >> 8);
            add = swapped_operand;
            a = cpu.read_from_memory(swapped_operand, NO_PAGE);
            set_flags(a, cpu);
            p = NO_PAGE;
        },
        AddressMode::AbsoluteX => {
            let operand = cpu.consume_next_word();
            let swapped_operand : u16 = (((operand) & 0xff) << 8) | (((operand) & 0xff00) >> 8);
            add = swapped_operand + cpu.x as u16;
            a = cpu.read_from_memory(swapped_operand + cpu.x as u16, NO_PAGE);
            set_flags(a, cpu);
            p = NO_PAGE;
        },
        _ => println!("Invalid address mode")
    }
    cpu.write_to_memory(a - 1, add, p);
    1
}

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn dex(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16a : u16 = a as u16 & 0x00ff;

        let r : i16 = u16a as i16 - 1;
        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
    }

    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.x -= 1;
            set_flags(cpu.x, cpu);
        },
        _ => println!("Invalid address mode")
    }
    1
}

#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn dey(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16a : u16 = a as u16 & 0x00ff;

        let r : i16 = u16a as i16 - 1;
        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
    }

    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.y -= 1;
            set_flags(cpu.y, cpu);
        },
        _ => println!("Invalid address mode")
    }
    1
}