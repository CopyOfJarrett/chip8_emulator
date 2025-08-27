const MEM_SIZE: usize = 4096;

pub struct Memory {
    ram: [u8; MEM_SIZE], /* 4kB or 4096 Bytes of Addressable Memory (RAM) */
}

impl Memory {
    pub fn init() -> Self {
        /* Initalize Memory */
        Memory { ram: [0; MEM_SIZE] }
    }
    pub fn read_ram(&self, address: usize) -> u8 {
        /* Reads and Returns value at RAM location [Address] */
        self.ram[address]
    }
    pub fn write_ram(&mut self, address: usize, value: u8) {
        /* Writes [value] to RAM location [Address] */
        self.ram[address] = value;
    }
}
