extern crate mold;

use mold::cpu::*;
use mold::opcodes::opcodes::*;
use mold::opcodes::and::*;

fn utility_initialize_cpu() -> Cpu {
    Cpu::new()
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_immediate(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Immediate, and);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.memory.bank[0] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));

    // negative
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.memory.bank[0] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    // zero
    cpu.pc = 0;
    cpu.ac = 0x70;
    cpu.memory.bank[0] = 0x07;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_zeropage(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPage, and);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));

    // negative
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    // zero
    cpu.pc = 0;
    cpu.ac = 0x70;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x07;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_zeropage_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPageX, and);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // negative
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    // zero
    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0xF0;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_absolute(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Absolute, and);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // negative
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    // zero
    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0xF0;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_absolute_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::AbsoluteX, and);

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0xF0;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_absolute_y(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::AbsoluteY, and);

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0xF0;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_indirect_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::IndirectX, and);

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x10] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x03);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x10] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x10] = 0xF0;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_and_indirect_y(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::IndirectY, and);

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x0F] = 0x03;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x04);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.ac = 0x80;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x0F] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    cpu.pc = 0;
    cpu.ac = 0x0F;
    cpu.y  = 0x10;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x0F] = 0xE0;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
}
