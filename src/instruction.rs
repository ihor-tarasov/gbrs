use crate::{Bus, Cpu};

pub fn execute(cpu: &mut Cpu, bus: &mut Bus) -> u32 {
    match bus.read(cpu.pc) {
        0x31 => ld_sp_n16(cpu, bus),
        opcode => panic!("Unimplemented opcode: 0x{opcode:02X} at 0x{:04X}", cpu.pc),
    }
}

// opcode:   0x31
// mnemonic: LD SP, n16
// T-cycles: 12
// Flags:    - - - -
fn ld_sp_n16(cpu: &mut Cpu, bus: &mut Bus) -> u32 {
    let n16 = bus.read_u16(cpu.pc.wrapping_add(1));
    println!("LD SP, 0x{n16:04X}");
    cpu.sp = n16;
    cpu.pc = cpu.pc.wrapping_add(3);
    12
}
