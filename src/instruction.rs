use crate::{Bus, Byte, Cpu, Error, Flags, Result, Word};

pub fn execute(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let opcode = bus.read(cpu.pc)?;
    match opcode.get() {
        0x21 => ld_hl_n16(cpu, bus),
        0x31 => ld_sp_n16(cpu, bus),
        0xAF => xor_a_a(cpu),
        _ => Err(Error::Opcode(opcode)),
    }
}

// opcode:     0x21
// mnemonic:   LD HL, n16
// T-cycles:   12
// Flags ZNHC: - - - -
fn ld_hl_n16(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let lo = bus.read(cpu.pc + 1)?;
    let hi = bus.read(cpu.pc + 2)?;
    log::debug!("{}: LD HL, {}", cpu.pc, Word::from_bytes(hi, lo));
    cpu.h = hi;
    cpu.l = lo;
    cpu.pc += 3;
    Ok(12)
}

// opcode:     0x31
// mnemonic:   LD SP, n16
// T-cycles:   12
// Flags ZNHC: - - - -
fn ld_sp_n16(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let lo = bus.read(cpu.pc + 1)?;
    let hi = bus.read(cpu.pc + 2)?;
    let n16 = Word::from_bytes(hi, lo);
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
