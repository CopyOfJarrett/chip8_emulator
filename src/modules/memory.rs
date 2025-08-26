const MEM_SIZE: usize = 4096;

pub struct Memory {
    ram: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Memory { ram: [0; MEM_SIZE] }
    }
    pub fn read_ram(&self, address: usize) -> u8 {
        self.ram[address]
    }
    pub fn write_ram(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }
}
