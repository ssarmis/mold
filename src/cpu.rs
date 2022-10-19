use crate::opcodes::adc::*;
use crate::opcodes::and::*;
use crate::opcodes::branch::*;
use crate::opcodes::cmp::*;
use crate::opcodes::dec::*;
use crate::opcodes::eor::*;
use crate::opcodes::inc::*;
use crate::opcodes::jmp::*;
use crate::opcodes::ld::*;
use crate::opcodes::opcodes::*;
use crate::opcodes::ora::*;
use crate::opcodes::pp::*;
use crate::opcodes::ret::*;
use crate::opcodes::rotate::*;
use crate::opcodes::sbc::*;
use crate::opcodes::set::*;
use crate::opcodes::shift::*;
use std::fmt;

#[derive(Clone,Copy,Debug)]
pub struct Memory {
    pub bank : [i8; 0x4000],
    pub page_offsets:[u16; 3]
}

// for us these don't really mean anything, 
// but in case we want to change how the cpu reads
// from these pages, we can se custom offsets
pub const NO_PAGE : u16 = 0;
pub const ZERO_PAGE : u16 = 1; 
pub const STACK_PAGE : u16 = 2;

#[allow(dead_code)]
impl Memory {
    pub fn new() -> Memory {
        Memory {
            bank: [0; 0x4000],
            page_offsets: [0, 0, 0x0100]
        }
    }

    fn read8(&mut self, address : u16, page:u16) -> i8 {
        // add error handling
        let offset = self.page_offsets[page as usize] + address;
        self.bank[offset as usize]
    }

    #[allow(arithmetic_overflow)]
    fn read16(&mut self, address : u16, page:u16) -> u16 {
        // add error handling
        
        let offset = (self.page_offsets[page as usize] + address) as usize;

        let result : u16 = ((self.bank[offset as usize] as u16) << 8) | 
                            (self.bank[(offset + 1) as usize] as u16);
        result
    }

    fn write8(&mut self, value : i8, address : u16, page:u16) {
        // add error handling
        let offset = (self.page_offsets[page as usize] + address) as usize;
        self.bank[offset] = value;
    }
}

#[allow(dead_code)]
pub struct Cpu {
    pub pc:u16,
    pub sp:u8,

    pub n:u8,
    pub v:u8,
    pub b:u8,
    pub d:u8,
    pub i:u8,
    pub z:u8,
    pub c:u8,

    pub ac:i8,
    pub x:i8,
    pub y:i8,

    pub instructions : [Opcode; 255],
    pub memory: Memory
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f,
        //     "{}", self.x
        // )
        f.debug_struct("Cpu")
        .field("pc", &self.pc)
        .field("sp", &self.sp)
        .field("n", &self.n)
        .field("v", &self.v)
        .field("b", &self.b)
        .field("d", &self.d)
        .field("i", &self.i)
        .field("z", &self.z)
        .field("c", &self.c)
        .field("ac", &self.ac)
        .field("x", &self.x)
        .field("y", &self.y)
        .finish()
    }
}

pub fn nopopcode() -> Opcode {
    Opcode::new(0, 1, AddressMode::NoMode, nop)
}

pub fn opn(func : fn(&Opcode, &mut Cpu) -> u8, a : AddressMode, bytes : i8, c : i8) -> Opcode {
    Opcode::new(0, c, a, func)
}

#[allow(dead_code)]
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            ac: 0,
            n: 0,
            v: 0,
            b: 0,
            d: 0,
            i: 0,
            z: 0,
            c: 0,
            x: 0,
            y: 0,
            instructions: generate_instruction_array(),
            memory: Memory::new()
        }
    }

    pub fn consume_next_byte(&mut self) -> i8 {
        let result = self.memory.read8(self.pc, NO_PAGE);
        self.pc += 1;
        result
    }

    pub fn consume_next_word(&mut self) -> u16 {
        let result = self.memory.read16(self.pc, NO_PAGE);
        self.pc += 2;
        result
    }

    pub fn read_from_memory(&mut self, address: u16, page: u16) -> i8 {
        let result = self.memory.read8(address, page);
        result
    }

    pub fn read_word_from_memory(&mut self, address: u16, page: u16) -> u16 {
        let result = self.memory.read16(address, page);
        result
    }
    
    pub fn write_to_memory(&mut self, value : i8, address: u16, page: u16){
        self.memory.write8(value, address, page);
    }

    pub fn write_vec_to_memory(&mut self, array : Vec<u8>, address: u16, page: u16){
        let mut offset = 0;
        for byte in array.iter() {
            self.memory.write8(*byte as i8, address + offset, page);
            offset += 1;
        }
    }

    pub fn push_to_stack(&mut self, value : i8){
        self.memory.write8(value, self.sp as u16, STACK_PAGE);
        self.sp += 1;
    }

    pub fn pop_from_stack(&mut self) -> i8 {
        let result = self.memory.read8(self.sp as u16, STACK_PAGE);
        self.sp -= 1;
        result
    }
}

pub fn generate_instruction_array() -> [Opcode; 255] {
    let mut instructions : [Opcode; 255] = [opn(nop, AddressMode::Implied, 1, 1); 255];

    instructions[0x69] = opn(adc,AddressMode::Immediate,2,2);
    instructions[0x65] = opn(adc,AddressMode::ZeroPage,2,3);
    instructions[0x75] = opn(adc,AddressMode::ZeroPageX,2,4);
    instructions[0x6d] = opn(adc,AddressMode::Absolute,3,4);
    instructions[0x7d] = opn(adc,AddressMode::AbsoluteX,3,4);
    instructions[0x79] = opn(adc,AddressMode::AbsoluteY,3,4);
    instructions[0x61] = opn(adc,AddressMode::IndirectX,2,6);
    instructions[0x71] = opn(adc,AddressMode::IndirectY,2,5);
    instructions[0x29] = opn(and,AddressMode::Immediate,2,2);
    instructions[0x25] = opn(and,AddressMode::ZeroPage,2,3);
    instructions[0x35] = opn(and,AddressMode::ZeroPageX,2,4);
    instructions[0x2d] = opn(and,AddressMode::Absolute,3,4);
    instructions[0x3d] = opn(and,AddressMode::AbsoluteX,3,4);
    instructions[0x39] = opn(and,AddressMode::AbsoluteY,3,4);
    instructions[0x21] = opn(and,AddressMode::IndirectX,2,6);
    instructions[0x31] = opn(and,AddressMode::IndirectY,2,5);
    instructions[0x0a] = opn(asl,AddressMode::Accumulator,1,2);
    instructions[0x06] = opn(asl,AddressMode::ZeroPage,2,5);
    instructions[0x16] = opn(asl,AddressMode::ZeroPageX,2,6);
    instructions[0x0e] = opn(asl,AddressMode::Absolute,3,6);
    instructions[0x1e] = opn(asl,AddressMode::AbsoluteX,3,7);
    instructions[0x90] = opn(bcc,AddressMode::Relative,2,2);//3
    instructions[0xB0] = opn(bcs,AddressMode::Relative,2,2);//3
    instructions[0xF0] = opn(beq,AddressMode::Relative,2,2);//3
    instructions[0x30] = opn(bmi,AddressMode::Relative,2,2);//3
    instructions[0xD0] = opn(bne,AddressMode::Relative,2,2);//3
    instructions[0x10] = opn(bpl,AddressMode::Relative,2,2);//3
    instructions[0x50] = opn(bvc,AddressMode::Relative,2,2);//3
    instructions[0x70] = opn(bvs,AddressMode::Relative,2,2);//3
    instructions[0x24] = opn(bit,AddressMode::ZeroPage,2,3);
    instructions[0x2c] = opn(bit,AddressMode::Absolute,3,4);
    instructions[0x00] = opn(brk,AddressMode::Implied,1,7);
    instructions[0x18] = opn(clc,AddressMode::Implied,1,2);
    instructions[0xd8] = opn(cld,AddressMode::Implied,1,2);
    instructions[0x58] = opn(cli,AddressMode::Implied,1,2);
    instructions[0xb8] = opn(clv,AddressMode::Implied,1,2);
    instructions[0xea] = opn(nop,AddressMode::Implied,1,2);
    instructions[0x48] = opn(pha,AddressMode::Implied,1,3);
    instructions[0x68] = opn(pla,AddressMode::Implied,1,4);
    instructions[0x08] = opn(php,AddressMode::Implied,1,3);
    instructions[0x28] = opn(plp,AddressMode::Implied,1,4);
    instructions[0x40] = opn(rti,AddressMode::Implied,1,6);
    instructions[0x60] = opn(rts,AddressMode::Implied,1,6);
    instructions[0x38] = opn(sec,AddressMode::Implied,1,2);
    instructions[0xf8] = opn(sed,AddressMode::Implied,1,2);
    instructions[0x78] = opn(sei,AddressMode::Implied,1,2);
    instructions[0xaa] = opn(tax,AddressMode::Implied,1,2);
    instructions[0x8a] = opn(txa,AddressMode::Implied,1,2);
    instructions[0xa8] = opn(tay,AddressMode::Implied,1,2);
    instructions[0x98] = opn(tya,AddressMode::Implied,1,2);
    instructions[0xba] = opn(tsx,AddressMode::Implied,1,2);
    instructions[0x9a] = opn(txs,AddressMode::Implied,1,2);
    instructions[0xc9] = opn(cmp,AddressMode::Immediate,2,2);
    instructions[0xc5] = opn(cmp,AddressMode::ZeroPage,2,3);
    instructions[0xd5] = opn(cmp,AddressMode::ZeroPageX,2,4);
    instructions[0xcd] = opn(cmp,AddressMode::Absolute,3,4);
    instructions[0xdd] = opn(cmp,AddressMode::AbsoluteX,3,4);
    instructions[0xd9] = opn(cmp,AddressMode::AbsoluteY,3,4);
    instructions[0xc1] = opn(cmp,AddressMode::IndirectX,2,6);
    instructions[0xd1] = opn(cmp,AddressMode::IndirectY,2,5);
    instructions[0xe0] = opn(cpx,AddressMode::Immediate,2,2);
    instructions[0xe4] = opn(cpx,AddressMode::ZeroPage,2,3);
    instructions[0xec] = opn(cpx,AddressMode::Absolute,3,4);
    instructions[0xc0] = opn(cpy,AddressMode::Immediate,2,2);
    instructions[0xc4] = opn(cpy,AddressMode::ZeroPage,2,3);
    instructions[0xcc] = opn(cpy,AddressMode::Absolute,3,4);
    instructions[0xc6] = opn(dec,AddressMode::ZeroPage,2,5);
    instructions[0xd6] = opn(dec,AddressMode::ZeroPageX,2,6);
    instructions[0xce] = opn(dec,AddressMode::Absolute,3,6);
    instructions[0xde] = opn(dec,AddressMode::AbsoluteX,3,7);
    instructions[0xca] = opn(dex,AddressMode::Implied,1,2);
    instructions[0x88] = opn(dey,AddressMode::Implied,1,2);
    instructions[0xe8] = opn(inx,AddressMode::Implied,1,2);
    instructions[0xc8] = opn(iny,AddressMode::Implied,1,2);
    instructions[0x49] = opn(eor,AddressMode::Immediate,2,2);
    instructions[0x45] = opn(eor,AddressMode::ZeroPage,2,3);
    instructions[0x55] = opn(eor,AddressMode::ZeroPageX,2,4);
    instructions[0x4d] = opn(eor,AddressMode::Absolute,3,4);
    instructions[0x5d] = opn(eor,AddressMode::AbsoluteX,3,4);
    instructions[0x59] = opn(eor,AddressMode::AbsoluteY,3,4);
    instructions[0x41] = opn(eor,AddressMode::IndirectX,2,6);
    instructions[0x51] = opn(eor,AddressMode::IndirectY,2,5);
    instructions[0xe6] = opn(inc,AddressMode::ZeroPage,2,5);
    instructions[0xf6] = opn(inc,AddressMode::ZeroPageX,2,6);
    instructions[0xee] = opn(inc,AddressMode::Absolute,3,6);
    instructions[0xfe] = opn(inc,AddressMode::AbsoluteX,3,7);
    instructions[0x4c] = opn(jmp,AddressMode::Absolute,3,3);
    instructions[0x6c] = opn(jmp,AddressMode::Indirect,3,5);
    instructions[0x20] = opn(jsr,AddressMode::Absolute,3,6);
    instructions[0xa9] = opn(lda,AddressMode::Immediate,2,2);
    instructions[0xa5] = opn(lda,AddressMode::ZeroPage,2,3);
    instructions[0xb5] = opn(lda,AddressMode::ZeroPageX,2,4);
    instructions[0xad] = opn(lda,AddressMode::Absolute,3,4);
    instructions[0xbd] = opn(lda,AddressMode::AbsoluteX,3,4);
    instructions[0xb9] = opn(lda,AddressMode::AbsoluteY,3,4);
    instructions[0xa1] = opn(lda,AddressMode::IndirectX,2,6);
    instructions[0xb1] = opn(lda,AddressMode::IndirectY,2,5);
    instructions[0xa2] = opn(ldx,AddressMode::Immediate,2,2);
    instructions[0xa6] = opn(ldx,AddressMode::ZeroPage,2,3);
    instructions[0xb6] = opn(ldx,AddressMode::ZeroPageY,2,4);
    instructions[0xae] = opn(ldx,AddressMode::Absolute,3,4);
    instructions[0xbe] = opn(ldx,AddressMode::AbsoluteY,3,4);
    instructions[0xa0] = opn(ldy,AddressMode::Immediate,2,2);
    instructions[0xa4] = opn(ldy,AddressMode::ZeroPage,2,3);
    instructions[0xb4] = opn(ldy,AddressMode::ZeroPageX,2,4);
    instructions[0xac] = opn(ldy,AddressMode::Absolute,3,4);
    instructions[0xbc] = opn(ldy,AddressMode::AbsoluteX,3,4);
    instructions[0x4a] = opn(lsr,AddressMode::Accumulator,1,2);
    instructions[0x46] = opn(lsr,AddressMode::ZeroPage,2,5);
    instructions[0x56] = opn(lsr,AddressMode::ZeroPageX,2,6);
    instructions[0x4e] = opn(lsr,AddressMode::Absolute,3,6);
    instructions[0x5e] = opn(lsr,AddressMode::AbsoluteX,3,7);
    instructions[0x09] = opn(ora,AddressMode::Immediate,2,2);
    instructions[0x05] = opn(ora,AddressMode::ZeroPage,2,3);
    instructions[0x15] = opn(ora,AddressMode::ZeroPageX,2,4);
    instructions[0x0d] = opn(ora,AddressMode::Absolute,3,4);
    instructions[0x1d] = opn(ora,AddressMode::AbsoluteX,3,4);
    instructions[0x19] = opn(ora,AddressMode::AbsoluteY,3,4);
    instructions[0x01] = opn(ora,AddressMode::IndirectX,2,6);
    instructions[0x11] = opn(ora,AddressMode::IndirectY,2,5);
    instructions[0x2a] = opn(rol,AddressMode::Accumulator,1,2);
    instructions[0x26] = opn(rol,AddressMode::ZeroPage,2,5);
    instructions[0x36] = opn(rol,AddressMode::ZeroPageX,2,6);
    instructions[0x2e] = opn(rol,AddressMode::Absolute,3,6);
    instructions[0x3e] = opn(rol,AddressMode::AbsoluteX,3,7);
    instructions[0x6a] = opn(ror,AddressMode::Accumulator,1,2);
    instructions[0x66] = opn(ror,AddressMode::ZeroPage,2,5);
    instructions[0x76] = opn(ror,AddressMode::ZeroPageX,2,6);
    instructions[0x7e] = opn(ror,AddressMode::Absolute,3,6);
    instructions[0x6e] = opn(ror,AddressMode::AbsoluteX,3,7);
    instructions[0xe9] = opn(sbc,AddressMode::Immediate,2,2);
    instructions[0xe5] = opn(sbc,AddressMode::ZeroPage,2,3);
    instructions[0xf5] = opn(sbc,AddressMode::ZeroPageX,2,4);
    instructions[0xed] = opn(sbc,AddressMode::Absolute,3,4);
    instructions[0xfd] = opn(sbc,AddressMode::AbsoluteX,3,4);
    instructions[0xf9] = opn(sbc,AddressMode::AbsoluteY,3,4);
    instructions[0xe1] = opn(sbc,AddressMode::IndirectX,2,6);
    instructions[0xf1] = opn(sbc,AddressMode::IndirectY,2,5);
    instructions[0x85] = opn(sta,AddressMode::ZeroPage,2,3);
    instructions[0x95] = opn(sta,AddressMode::ZeroPageX,2,4);
    instructions[0x8d] = opn(sta,AddressMode::Absolute,3,4);
    instructions[0x9d] = opn(sta,AddressMode::AbsoluteX,3,5);
    instructions[0x99] = opn(sta,AddressMode::AbsoluteY,3,5);
    instructions[0x81] = opn(sta,AddressMode::IndirectX,2,6);
    instructions[0x91] = opn(sta,AddressMode::IndirectY,2,6);
    instructions[0x86] = opn(stx,AddressMode::ZeroPage,2,3);
    instructions[0x96] = opn(stx,AddressMode::ZeroPageY,2,4);
    instructions[0x8e] = opn(stx,AddressMode::Absolute,3,4);
    instructions[0x84] = opn(stx,AddressMode::ZeroPage,2,3);
    instructions[0x94] = opn(stx,AddressMode::ZeroPageX,2,4);
    instructions[0x8c] = opn(stx,AddressMode::Absolute,3,4);
    instructions
}


#[allow(dead_code)]
pub fn flags_to_sr(n : u8, v : u8, _unused : i8, b : u8, d : u8, i : u8, z : u8, c : u8) -> u8{
    let mut result = 0;
    result |= n << 7;
    result |= v << 6;
    result |= 0 << 5;
    result |= b << 4;
    result |= d << 3;
    result |= i << 2;
    result |= z << 1;
    result |= c << 0;
    result
}

#[allow(dead_code)]
pub fn cpu_flags_to_sr(cpu : &Cpu) -> u8{
    let mut result = 0;
    result |= cpu.n << 7;
    result |= cpu.v << 6;
    result |= 0     << 5;
    result |= cpu.b << 4;
    result |= cpu.d << 3;
    result |= cpu.i << 2;
    result |= cpu.z << 1;
    result |= cpu.c << 0;
    result
}

pub fn cpu_sr_from_u8(cpu : &mut Cpu, value : i8){
    cpu.n = value as u8 >> 7;
    cpu.v = value as u8 >> 6;
    //     = value >> 5 
    cpu.b = value as u8 >> 4;   
    cpu.d = value as u8 >> 3;
    cpu.i = value as u8 >> 2;
    cpu.z = value as u8 >> 1;
    cpu.c = value as u8 >> 0;
}