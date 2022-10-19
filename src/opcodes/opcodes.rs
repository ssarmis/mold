use crate::cpu::{Cpu, ZERO_PAGE, NO_PAGE};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum AddressMode {
    NoMode,
    Relative,
    Accumulator,
    Immediate,
    Absolute, 
    AbsoluteX, 
    AbsoluteY, 
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY
}


#[derive(Copy, Clone)]
pub struct Opcode {
    pub size: u8,
    pub cycles : i8,
    pub address_mode: AddressMode,
    pub execute: fn(&Opcode, &mut Cpu) -> u8
}

impl Opcode {
    // pub fn new(opsize:u8, addmode: AddressMode, opfunc: fn(&Opcode, &mut Cpu) -> u8) -> Opcode {
    //     Opcode {
    //         size:opsize,
    //         cycles:0,
    //         address_mode:addmode,
    //         execute:opfunc
    //     }
    // }
    pub fn new(opsize:u8, c:i8, addmode: AddressMode, opfunc: fn(&Opcode, &mut Cpu) -> u8) -> Opcode {
        Opcode {
            size:opsize,
            cycles:c,
            address_mode:addmode,
            execute:opfunc
        }
    }
}

pub fn nop(_opcode:&Opcode, _cpu:&mut Cpu) -> u8{
    1
}

pub fn u8u16add(a : i8, b : i8) -> u16{
    let u16a : u16 = a as u16 & 0x00ff;
    let u16b : u16 = b as u16 & 0x00ff;
    u16a + u16b
}

pub fn immediate_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_byte();
    operand
}

pub fn zero_page_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_byte();
    let a = cpu.read_from_memory(operand as u16, ZERO_PAGE);
    a
}

pub fn zero_page_store(value : i8, cpu : &mut Cpu){
    let operand = cpu.consume_next_byte();
    let a = cpu.write_to_memory(value, operand as u16, ZERO_PAGE);
    a
}

#[allow(overflowing_literals)]
pub fn zero_page_x_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_byte();
    let address = (operand + cpu.x) & 0xFF;
    let a = cpu.read_from_memory(address as u16, ZERO_PAGE);
    a
}

#[allow(overflowing_literals)]
pub fn zero_page_x_store(value : i8, cpu : &mut Cpu) {
    let operand = cpu.consume_next_byte();
    let address = (operand + cpu.x) & 0xFF;
    let a = cpu.write_to_memory(value, address as u16, ZERO_PAGE);
}

pub fn absolute_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_word();
    let swapped_operand : u16 = (((operand) & 0xff) << 8) | (((operand) & 0xff00) >> 8);
    let a = cpu.read_from_memory(swapped_operand, NO_PAGE);
    a
}

pub fn absolute_x_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_word();
    let swapped_operand = ((operand & 0xff) << 8) | ((operand & 0xff00) >> 8);
    let a = cpu.read_from_memory(swapped_operand + cpu.x as u16, NO_PAGE);
    a
}

pub fn absolute_y_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_word();
    let swapped_operand = ((operand & 0xff) << 8) | ((operand & 0xff00) >> 8);
    let a = cpu.read_from_memory(swapped_operand + cpu.y as u16, NO_PAGE);
    a
}

pub fn absolute_store(value : i8, cpu : &mut Cpu) {
    let operand = cpu.consume_next_word();
    let swapped_operand : u16 = (((operand) & 0xff) << 8) | (((operand) & 0xff00) >> 8);
    cpu.write_to_memory(value, swapped_operand, NO_PAGE);
}

pub fn absolute_x_store(value :i8, cpu : &mut Cpu) {
    let operand = cpu.consume_next_word();
    let swapped_operand = ((operand & 0xff) << 8) | ((operand & 0xff00) >> 8);
    cpu.write_to_memory(value, swapped_operand + cpu.x as u16, NO_PAGE);
}

pub fn absolute_y_store(value :i8, cpu : &mut Cpu) {
    let operand = cpu.consume_next_word();
    let swapped_operand = ((operand & 0xff) << 8) | ((operand & 0xff00) >> 8);
    cpu.write_to_memory(value, swapped_operand + cpu.y as u16, NO_PAGE);
}


#[allow(overflowing_literals)]
pub fn indirect_x_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_byte();
    let a = cpu.read_from_memory(((operand + cpu.x) & 0xff) as u16, ZERO_PAGE);
    a
}

pub fn indirect_y_fetch(cpu : &mut Cpu) -> i8 {
    let operand = cpu.consume_next_byte();
    let value_address = cpu.read_from_memory(operand as u16, ZERO_PAGE);
    let a = value_address + cpu.y;
    a
}

#[allow(overflowing_literals)]
pub fn indirect_x_store(value: i8, cpu : &mut Cpu) {
    let operand = cpu.consume_next_byte();
    cpu.write_to_memory(value, ((operand + cpu.x) & 0xff) as u16, ZERO_PAGE);
}

pub fn indirect_y_store(value : i8, cpu : &mut Cpu){
     // ????? how the fuck?????
    let operand = cpu.consume_next_byte();
    let value_address = cpu.write_to_memory(value, operand as u16, ZERO_PAGE);
    // let a = value_address + cpu.y;
}