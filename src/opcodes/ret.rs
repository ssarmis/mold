use crate::opcodes::opcodes::*;
use crate::cpu::*;

pub fn rti(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        _cpu.n = if a < 0 {1} else {0};
        _cpu.z = if a == 0 {1} else {0};
    }
    match opcode.address_mode {
        AddressMode::Implied => {
            let mut operand = cpu.pop_from_stack();

            set_flags(operand, cpu);
            cpu_sr_from_u8(cpu, operand);

            let h0 = cpu.pop_from_stack();
            let h1 = cpu.pop_from_stack();
            cpu.pc = ((h0 as u16) << 8) | (h1 as u16);
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn rts(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        _cpu.n = if a < 0 {1} else {0};
        _cpu.z = if a == 0 {1} else {0};
    }
    match opcode.address_mode {
        AddressMode::Implied => {
            let mut operand = cpu.pop_from_stack();

            let h0 = cpu.pop_from_stack();
            let h1 = cpu.pop_from_stack();
            cpu.pc = ((h0 as u16) << 8) | (h1 as u16);

            cpu.pc += 1;
        },
        _ => println!("Invalid address mode")
    }
    1
}
