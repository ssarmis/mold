use crate::opcodes::opcodes::*;
use crate::cpu::*;

pub fn bcc(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.c == 0 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn bcs(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.c == 1 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn beq(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.z == 1 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

// this isnt a test, but, was to lazy to move it from here
pub fn bit(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    let mut operand : i8 = 0;
    match opcode.address_mode {
        AddressMode::ZeroPage => {
            operand = zero_page_fetch(cpu);
        },
        AddressMode::Absolute => {
            operand = absolute_fetch(cpu);
        },
        _ => println!("Invalid address mode")
    }
    let bit7 = 0x40;
    let bit6 = 0x20;
    cpu.n = operand as u8 & bit7;
    cpu.v = operand as u8 & bit6;
    let result = cpu.ac & operand;
    cpu.z = if result == 0 {1} else {0};
    1
}

pub fn bmi(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.n == 1 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn bne(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.z == 0 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn bpl(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.n == 0 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn brk(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            // implement interrupt 
            cpu.i = 1;
            let half0 = (((cpu.pc + 2) & 0xff00) >> 8) as i8;
            let half1 = (((cpu.pc + 2) & 0x00ff) >> 8) as i8;

            cpu.push_to_stack(half1);
            cpu.push_to_stack(half0);
            cpu.push_to_stack(cpu_flags_to_sr(cpu) as i8);
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn bvc(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.v == 0 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn bvs(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Relative => {
            if cpu.v == 1 {
                let operand = immediate_fetch(cpu);
                cpu.pc = (cpu.pc as i16 + operand as i16) as u16;
            }
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn clc(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.c = 0;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn cld(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.d = 0;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn cli(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.i = 0;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn clv(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.v = 0;
        },
        _ => println!("Invalid address mode")
    }
    1
}
