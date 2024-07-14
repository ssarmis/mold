# mold
A project made just for learning some Rust🤮<br>
Its a, hopefully, partially, functioning CPU emulator<br>
(basically only the opcodes are being implemented, partially tested), not cycle-accurate

You can use the mold::cpu in order to execute opcodes
A basic example is the following
```cpp
let mut cpu = Cpu::new();

cpu.write_vec_to_memory(vec![
    0xA2, 0x00, 0xBD, 0x00, 0x10, 0x9D, 0x00, 0x20
], 0x0000, NO_PAGE);
cpu.write_vec_to_memory(vec![
    0xE8, 0xE0, 0xE8, 0xD0, 0xF5
], 0x0008, NO_PAGE);

cpu.pc = 0x0000;

while true { // obviously a break condition will be needed at some point
    let op = cpu.consume_next_byte() as u8;
    let instruction = cpu.instructions[op as usize];
    (cpu.instructions[op as usize].execute)(&instruction, &mut cpu);
}
```

Created by Streanga Sarmis-Stefan
