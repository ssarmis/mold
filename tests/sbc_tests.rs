extern crate mold;

use mold::cpu::*;
use mold::opcodes::opcodes::*;
use mold::opcodes::sbc::*;

fn utility_initialize_cpu() -> Cpu {
    Cpu::new()
}

#[test]
#[allow(overflowing_literals)]
#[allow(arithmetic_overflow)]
pub fn test_sbc_immediate(){
    let mut cpu = utility_initialize_cpu();
    let opcode = Opcode::new(1, AddressMode::Immediate, sbc);

     // normal
     cpu.pc = 0;
     cpu.ac = 0x01;
     cpu.memory.bank[0] = 0x01;
     (opcode.execute)(&opcode, &mut cpu);
     assert_eq!(cpu.ac, 0x00);
     assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 0));
 
     cpu.pc = 0;
     cpu.ac = 0x50;
     cpu.memory.bank[0] = 0xB0;
     (opcode.execute)(&opcode, &mut cpu);
     assert_eq!(cpu.ac, 0xA0);
     assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(1, 0, 0, 0, 0, 0, 0, 0));

    // // carry
    // cpu.pc = 0;
    // cpu.ac = 0xFF;
    // cpu.memory.bank[0] = 0x01;
    // (opcode.execute)(&opcode, &mut cpu);
    // assert_eq!(cpu.ac, 0x00);
    // assert_eq!(cpu_flags_to_sr(&cpu), flags_to_sr(0, 0, 0, 0, 0, 0, 1, 1));
}
