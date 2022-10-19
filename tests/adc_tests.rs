extern crate mold;

use mold::cpu::*;
use mold::opcodes::opcodes::*;
use mold::opcodes::adc::*;

fn utility_initialize_cpu() -> Cpu {
    Cpu::new()
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_immediate(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Immediate, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.memory.bank[0] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));

    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.memory.bank[0] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.memory.bank[0] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_zeropage(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPage, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_zeropage_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::ZeroPageX, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[0x0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_absolute(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Absolute, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0A] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_absolute_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::AbsoluteX, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_absolute_y(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::AbsoluteY, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0A;
    cpu.memory.bank[1] = 0x0F;
    cpu.memory.bank[0x0F0B] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_indirect_x(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::IndirectX, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x10] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x10] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.x  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x10] = 0x01;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}

#[test]
#[allow(overflowing_literals)]
pub fn test_adc_indirect_y(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::IndirectY, adc);

    // normal
    cpu.pc = 0;
    cpu.ac = 0x01;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x0F] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x02);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 0, 0));
    
    // overflow, 127 + 1 -> -128
    cpu.pc = 0;
    cpu.ac = 0x7F;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x0F] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, -0x80);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 1, 0, 0, 0, 0, 0, 0));

    // carry, 255 + 1 -> 256
    cpu.pc = 0;
    cpu.ac = 0xFF;
    cpu.y  = 0x01;
    cpu.memory.bank[0] = 0x0F;
    cpu.memory.bank[0x0F] = 0x00;
    (opcode.execute)(&opcode, &mut cpu);
    assert_eq!(cpu.ac, 0x00);
    assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}
