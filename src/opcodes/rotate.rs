use crate::opcodes::opcodes::*;
use crate::cpu::*;

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn rol(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let mut u16a  : u16 = a as u16 & 0x00ff;

        let lb = (a & 0x80) >> 7;
        u16a <<= 1;
        u16a |= lb as u16;

        let r8 = (u16a & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if u16a > 0xff {1} else {0};
    }

    let mut a : i8 = -1;
    let mut add : u16 = 0;
    let mut p : u16 = 0;

    match opcode.address_mode {
        AddressMode::Accumulator => {
            set_flags(cpu.ac, cpu);
            let lb = cpu.ac & 0x80;
            cpu.ac <<= 1;
            cpu.ac |= lb;
        },
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
    let lb = (a & 0x80) >> 7;
    a <<= 1;
    a |= lb;
    cpu.write_to_memory(a, add, p);
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn ror(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let mut u16a  : u16 = a as u16 & 0x00ff;

        let lb = (a & 0x80) >> 7;
        u16a >>= 1;
        u16a |= lb as u16;

        let r8 = (u16a & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if u16a > 0xff {1} else {0};
    }

    let mut operand : i8 = -1;
    let mut add : u16 = 0;
    let mut a : i8 = 0;
    let mut p : u16 = 0;

    match opcode.address_mode {
        AddressMode::Accumulator => {
            set_flags(cpu.ac, cpu);
            let lb = cpu.ac & 0x80;
            cpu.ac >>= 1;
            cpu.ac |= lb;
        },
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
    let lb = (a & 0x80) >> 7;
    a >>= 1;
    a |= lb;
    cpu.write_to_memory(a, add, p);
    1
}