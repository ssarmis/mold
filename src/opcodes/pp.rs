use crate::opcodes::opcodes::*;
use crate::cpu::*;

pub fn pha(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.push_to_stack(cpu.ac);
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn php(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.push_to_stack(cpu_flags_to_sr(cpu) as i8);
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn pla(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    fn set_flags(a: i8, _cpu: &mut Cpu){
        _cpu.n = if a < 0 {1} else {0};
        _cpu.z = if a == 0 {1} else {0};
    }
    match opcode.address_mode {
        AddressMode::Implied => {
            let operand = cpu.pop_from_stack();

            set_flags(operand, cpu);
            cpu.ac = operand;
        },
        _ => println!("Invalid address mode")
    }
    1
}


pub fn plp(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            let operand = cpu.pop_from_stack();
            cpu_sr_from_u8(cpu, operand);
        },
        _ => println!("Invalid address mode")
    }
    1
}
