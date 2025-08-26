use chip8_emulator::CPU;

fn main() {
    let mut cpu = CPU::new();
    let opcode = 0x8EF6;
    cpu.decode_execute(opcode);
}
