extern crate mold;

use mold::cpu::*;
use mold::opcodes::opcodes::*;
use mold::opcodes::dec::*;

fn utility_initialize_cpu() -> Cpu {
    Cpu::new()
}

#[test]
#[allow(overflowing_literals)]
pub fn test_dec_zeropage(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPage, dec);

    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0A], 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));

    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0A], 0xFF);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_dec_zeropage_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPageX, dec);

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0B], 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0B], 0xFF);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_dec_absolute(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Absolute, dec);

    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0F0A], 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));

    cpu.pc = 0;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0F0A], 0xFF);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_dec_absolute_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::AbsoluteX, dec);

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0F0B], 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));

    cpu.pc = 0;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.memory.bank[0x0F0B], 0xFF);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));
}
