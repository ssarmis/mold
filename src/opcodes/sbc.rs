use crate::opcodes::opcodes::*;
use crate::cpu::Cpu;

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn sbc(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    #[allow(overflowing_literals)]
    #[allow(arithmetic_overflow)]
    fn set_flags(a: i8, _cpu: &mut Cpu){
        let u16ac : i16 = (_cpu.ac as u16 & 0x00ff) as i16;
        let u16a  : i16 = (a as u16 & 0x00ff) as i16;

        // let r : i16 = ((u16ac + !u16a) as i16 + (1 - _cpu.c) as i16) as i16;
        let c = _cpu.c as i16;
        let r : i16 = (u16ac + (!u16a + 1) as i16 - c) as i16;

        let r8 = (r & 0xff) as i8;
        _cpu.n = if r8 < 0 {1} else {0};
        _cpu.z = if r8 == 0 {1} else {0};
        _cpu.c = if r > 0xff {1} else {0};
        _cpu.v = (((_cpu.ac ^ r8) & !(_cpu.ac ^ a)) & 0x0080) as u8;
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
    let u16ac : u16 = cpu.ac as u16 & 0x00ff;
    let u16a  : u16 = operand as u16 & 0x00ff;
    // TODO(Sarmis) we can take the logic from set_flag and inline it here
    // also no need to duplicate it
    let r : i16 = ((u16ac + !u16a) as i16 + (1 - cpu.c) as i16) as i16;
    cpu.ac = (r & 0x00ff) as i8;
    1
}