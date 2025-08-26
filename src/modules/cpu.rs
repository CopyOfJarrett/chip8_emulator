use crate::modules::memory::Memory;

const NUM_REGS: usize = 16;
const START_ADDR: u16 = 0x200;

pub struct CPU {
    ram: Memory,
    v: [u8; NUM_REGS],      /*Vx Registers */
    i: u16,                 /* I or Index Register */
    stack: [u16; NUM_REGS], /* Stack, Pseudo Registers */
    sp: u8,                 /* Stack Pointer Register */
    pc: u16,                /* Program Counter Register */
    dt: u8,                 /* Delay Timer Register */
    st: u8,                 /* Sound Timer Register */
}

impl CPU {
    pub fn new() -> Self {
        /* Initialize Emulator */
        CPU {
            ram: Memory::new(),
            v: [0; NUM_REGS],
            i: 0,
            stack: [0; NUM_REGS],
            sp: 0,
            pc: START_ADDR, /* Program Counter = 0x200, default start address */
            dt: 0,
            st: 0,
        }
    }
    fn stack_push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }
    fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
    pub fn fetch(&self) -> u16 {
        todo!()
    }
    pub fn decode_execute(&mut self, instruction: u16) {
        let (nnn, x, y, n, nn) = (
            (instruction & 0xF000) >> 12, /*  F: Top Nibble (Instruction Family) */
            (instruction & 0x0F00) >> 8,  /*  X: Second Nibble (Vx Register) */
            (instruction & 0x00F0) >> 4,  /*  Y: Third Nibble (Vy Register) */
            instruction & 0x000F,         /*  N: Lowest Nibble */
            instruction & 0xFF,           /* NN: (KK, NN, Byte) Lowest Byte */
        );

        /* Default true, set false if when Program Counter is set (PC = Address) */
        let mut next_instruction = true;

        match (nnn, x, y, n) {
            /* NOP: Nothing */
            (0, 0, 0, 0) => {}
            /* 00E0: Clear the display */
            (0, 0, 0xE, 0) => {
                todo!();
            }
            /* 00EE: Return from subroutine */
            (0, 0, 0xE, 0xE) => {
                self.pc = self.stack_pop();

                next_instruction = false;
            }
            /* 1NNN: Jump to location NNN */
            (1, _, _, _) => {
                self.pc = nnn;

                next_instruction = false;
            }
            /* 2NNN: Call subroutine at location (NNN) */
            (2, _, _, _) => {
                self.stack_push(self.pc);
                self.pc = nnn;

                next_instruction = false;
            }
            /* 3XNN: Skip next instruction if Vx == NN then PC += 2 */
            (3, _, _, _) => {
                if self.v[x as usize] == nn as u8 {
                    self.pc += 2;
                }
            }
            /* 4XNN: Skip next instruction if Vx != NN then PC += 2 */
            (4, _, _, _) => {
                if self.v[x as usize] != nn as u8 {
                    self.pc += 2;
                }
            }
            /* 5XY0: Skip next instruction if Vx == Vy then PC += 2 */
            (5, _, _, 0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            /* 6XNN: Set Vx = NN */
            (6, _, _, _) => {
                self.v[x as usize] = nn as u8;
            }
            /* 7XKK: Set Vx = Vx + NN (Byte) */
            (7, _, _, _) => {
                self.v[x as usize] += nn as u8;
            }
            /* 8XY0: Set Vx = Vy */
            (8, _, _, 0) => {
                self.v[x as usize] = self.v[y as usize];
            }
            /* 8XY1: Set Vx = Vx OR Vy */
            (8, _, _, 1) => {
                self.v[x as usize] |= self.v[y as usize];
            }
            /* 8XY2: Set Vx = Vx AND Vy */
            (8, _, _, 2) => {
                self.v[x as usize] &= self.v[y as usize];
            }
            /* 8XY3: Set Vx = Vx XOR Vy */
            (8, _, _, 3) => {
                self.v[x as usize] ^= self.v[y as usize];
            }
            /* 8XY4: Set Vx = Vx + Vy, set VF = carry */
            (8, _, _, 4) => {
                let (result, carry) = self.v[x as usize].overflowing_add(self.v[y as usize]);

                self.v[x as usize] = result;
                self.v[0xF] = if carry { 1 } else { 0 };
            }
            /* 8XY5: Set Vx = Vx - Vy, set VF = NOT borrow */
            (8, _, _, 5) => {
                let (result, borrow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);

                self.v[x as usize] = result;
                self.v[0xF] = if borrow { 0 } else { 1 };
            }
            /* 8XY6 (SHR Vx): Set Vx = Vx >> 1, if Vx LSB = 1, set VF = 1 then Vx /= 2 */
            (8, _, _, 6) => {
                self.v[0xF] = self.v[x as usize] & 1;
                self.v[x as usize] >>= 1;
            }
            /* 8XY7 (SUBN Vx, Vy): Set Vx = Vy - Vx, if Vy > Vx, set VF = (NOT borrow) then Vx = Vx - Vy */
            (8, _, _, 7) => {
                let (result, borrow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);

                self.v[x as usize] = result;
                self.v[0xF] = if borrow { 0 } else { 1 };
            }
            /* 8XYE (SHL Vx): Set Vx = Vx << 1, if Vx MSB = 1, set VF = 1 then Vx *= 2 */
            (8, _, _, 0xE) => {
                self.v[0xF] = (self.v[x as usize] >> 7) & 0x1;
                self.v[x as usize] <<= 1;
            }
            /* 9XY0 (SNE Vx, Vy): Skip next instruction if Vx != Vy then PC += 2 */
            (9, _, _, 0) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            /* ANNN (LD I, Address): Set I = NNN (Addr) */
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            /* BNNN (JP V0, Address): Set I = NNN (Addr) + V0 */
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0] as u16;

                next_instruction = false;
            }
            /* CXKK (RND Vx, Byte): Set Vx = Random Byte & kk (Byte) */
            (0xC, _, _, _) => {
                let random_byte: u8 = rand::random_range(0..=255);

                self.v[x as usize] = random_byte & nn as u8;
            }
            /* DXYN: Display sprite starting at memory location I at (Vx, Vy), set VF = collision */
            (0xD, _, _, _) => {
                todo!();
            }
            /* EX9E: Skip next instruction if key with the value of Vx is pressed, if Vx = key-down then PC += 2 */
            (0xE, _, 9, 0xE) => {
                todo!();
            }
            /* EXA1: Skip next instruction if key with the value of Vx is NOT pressed, if Vx = key-up then PC += 2 */
            (0xE, _, 0xA, 1) => {
                todo!();
            }
            /* FX07 (LD Vx, DT): Set Vx = Delay Timer */
            (0xF, _, 0, 7) => {
                self.v[x as usize] = self.dt;
            }
            /* FX0A: Wait for key press then Vx = key */
            (0xF, _, 9, 0xA) => {
                todo!();
            }
            /* FX15 (LD DT, Vx): Set Delay Timer = Vx */
            (0xF, _, 1, 5) => {
                self.dt = self.v[x as usize];
            }
            /* FX18 (LD DT, Vx): Set Sound Timer = Vx */
            (0xF, _, 1, 0) => {
                self.st = self.v[x as usize];
            }
            /* FX1E (Add I, Vx): Set I = I + Vx  */
            (0xF, _, 1, 0xE) => {
                self.i += self.v[x as usize] as u16;
            }
            /* FX29: Set I = location of sprite for digit Vx */
            (0xF, _, 2, 9) => {
                todo!();
            }
            /* FX33 (LD B, Vx): I, I+1, and I+2 = BCD representation of Vx */
            (0xF, _, 3, 3) => {
                self.ram
                    .write_ram(self.i as usize, self.v[x as usize] / 100);
                self.ram
                    .write_ram(self.i as usize, (self.v[x as usize] / 10) % 10);
                self.ram.write_ram(self.i as usize, self.v[x as usize] % 10);
            }
            /* FX55 (LD [I], Vx): Ram[I],.. Ram[I + x] = V0,..Vx */
            (0xF, _, 5, 5) => {
                for idx in 0..x as usize {
                    self.ram.write_ram((self.i as usize) + idx, self.v[idx]);
                }
            }
            /* FX65 (LD Vx, [I]): V0,.. Vx = Ram[I],.. Ram[I + x] */
            (0xF, _, 6, 5) => {
                for idx in 0..x as usize {
                    self.v[idx] = self.ram.read_ram((self.i as usize) + idx);
                }
            }
            /* Unknown */
            (_, _, _, _) => unimplemented!("Unknown Instruction: {}", instruction),
        }

        /* If True, then Program Counter += 2*/
        if next_instruction {
            self.pc += 2;
        }
    }
}
