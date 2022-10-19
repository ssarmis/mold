use crate::opcodes::opcodes::*;
use crate::cpu::*;

pub fn sec(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.c = 1;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn sed(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.d = 1;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn sei(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.i = 1;
        },
        _ => println!("Invalid address mode")
    }
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn sta(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::ZeroPage => {
            zero_page_store(cpu.ac, cpu);
        },
        AddressMode::ZeroPageX => {
            zero_page_x_store(cpu.ac, cpu);
        },
        AddressMode::Absolute => {
            absolute_store(cpu.ac, cpu);
        },
        AddressMode::AbsoluteX => {
            absolute_x_store(cpu.ac, cpu);
        },
        AddressMode::AbsoluteY => {
            absolute_y_store(cpu.ac, cpu);
        },
        AddressMode::IndirectX => {
            indirect_x_store(cpu.ac, cpu);
        },
        AddressMode::IndirectY => {
            indirect_y_store(cpu.ac, cpu);
        },
        _ => println!("Invalid address mode")
    }
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn stx(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::ZeroPage => {
            zero_page_store(cpu.x, cpu);
        },
        AddressMode::ZeroPageX => {
            zero_page_x_store(cpu.x, cpu);
        },
        AddressMode::Absolute => {
            absolute_store(cpu.x, cpu);
        },
        _ => println!("Invalid address mode")
    }
    1
}

#[allow(unused_parens)]
#[allow(unused_assignments)]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn sty(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::ZeroPage => {
            zero_page_store(cpu.y, cpu);
        },
        AddressMode::ZeroPageX => {
            zero_page_x_store(cpu.y, cpu);
        },
        AddressMode::Absolute => {
            absolute_store(cpu.y, cpu);
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn tax(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.x = cpu.ac;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn tay(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.y = cpu.ac;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn tsx(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.x = cpu.sp as i8;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn txa(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.ac = cpu.x;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn txs(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.sp = cpu.x as u8;
        },
        _ => println!("Invalid address mode")
    }
    1
}

pub fn tya(opcode:&Opcode, cpu:&mut Cpu) -> u8 {
    match opcode.address_mode {
        AddressMode::Implied => {
            cpu.ac = cpu.y;
        },
        _ => println!("Invalid address mode")
    }
    1
}