use crate::{Bus, Byte, Cpu, Error, Flags, Result};

pub fn execute(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let opcode = bus.read_byte(cpu.pc)?;
    match opcode.get() {
        0x31 => ld_sp_n16(cpu, bus),
        0xAF => xor_a_a(cpu),
        _ => Err(Error::Opcode(opcode)),
    }
}

// opcode:     0x31
// mnemonic:   LD SP, n16
// T-cycles:   12
// Flags ZNHC: - - - -
fn ld_sp_n16(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let n16 = bus.read_word(cpu.pc + 1)?;
    log::debug!("{}: LD SP, {n16}", cpu.pc);
    cpu.sp = n16;
    cpu.pc += 3;
    Ok(12)
}

// opcode:     0xAF
// mnemonic:   XOR A, A
// T-cycles:   4
// Flags ZNHC: 1 0 0 0
fn xor_a_a(cpu: &mut Cpu) -> Result<u32> {
    log::debug!("{}: XOR A, A", cpu.pc);

    // A XOR A = 0
    cpu.a = Byte::ZERO;

    // Flags: Z N H C = 1 0 0 0
    cpu.f = Flags::new(true, false, false, false);

    cpu.pc += 1;
    Ok(4)
}
