extern crate mold;

use mold::cpu::*;
use mold::opcodes::opcodes::*;
use mold::opcodes::shift::*;

fn utility_initialize_cpu() -> Cpu {
    Cpu::new()
}

#[test]
#[allow(overflowing_literals)]
pub fn test_asl_accumulator(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Accumulator, asl);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x0F;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x1E);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));

    // carry + zero
    cpu.pc = 0;
    cpu.ac = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));

    // negative
    cpu.pc = 0;
    cpu.ac = 0x70;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0xE0);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_asl_zeropage(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPage, asl);

    // normal
    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x0F;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x1E);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));

    // carry + zero
    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));

    // negative
    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x70;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0xE0);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_asl_zeropage_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPageX, asl);

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x0F;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x1E);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x70;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0xE0);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_asl_absolute(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Absolute, asl);

    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x0F;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x1E);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));

    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x70;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0xE0);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_asl_absolute_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::AbsoluteX, asl);

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x0F;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x1E);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x80;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x70;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0xE0);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}
