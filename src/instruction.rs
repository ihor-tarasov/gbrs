use crate::{Bus, Byte, CPU, Error, Flag, Flags, Result, Word};

pub fn execute(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    let opcode = bus.read(cpu.pc)?;
    match opcode.get() {
        0x0E => ld_c_n8(cpu, bus),
        0x20 => jr_nz_e8(cpu, bus),
        0x21 => ld_hl_n16(cpu, bus),
        0x31 => ld_sp_n16(cpu, bus),
        0x32 => ld_hl_dec_a(cpu, bus),
        0x3E => ld_a_n8(cpu, bus),
        0xAF => xor_a_a(cpu),
        0xCB => cb(cpu, bus),
        _ => Err(Error::Opcode(opcode)),
    }
}

// opcode:     0x0E
// mnemonic:   LD C, n8
// T-cycles:   8
// Flags ZNHC: - - - -
fn ld_c_n8(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    let value = bus.read(cpu.pc + 1)?;
    log::debug!("{}: LD C, {value}", cpu.pc);
    cpu.c = value;
    cpu.pc += 2;
    Ok(8)
}

// opcode:     0x20
// mnemonic:   JR NZ, e8
// T-cycles:   12/8
// Flags ZNHC: - - - -
fn jr_nz_e8(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    let offset = bus.read(cpu.pc + 1)?.get() as i8;
    log::debug!("{}: JR NZ, {offset}", cpu.pc);
    if cpu.f.get(Flag::Zero) {
        cpu.pc += 2;
        Ok(8)
    } else {
        cpu.pc = Word::new((cpu.pc + 2).get().wrapping_add_signed(offset as i16));
        Ok(12)
    }
}

// opcode:     0x21
// mnemonic:   LD HL, n16
// T-cycles:   12
// Flags ZNHC: - - - -
fn ld_hl_n16(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
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
fn ld_sp_n16(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    let lo = bus.read(cpu.pc + 1)?;
    let hi = bus.read(cpu.pc + 2)?;
    let n16 = Word::from_bytes(hi, lo);
    log::debug!("{}: LD SP, {n16}", cpu.pc);
    cpu.sp = n16;
    cpu.pc += 3;
    Ok(12)
}

// opcode:     0x32
// mnemonic:   LD [HL-], A
// T-cycles:   8
// Flags ZNHC: - - - -
fn ld_hl_dec_a(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    log::debug!("{}: LD [HL-], A", cpu.pc);
    let address = cpu.hl();
    bus.write(address, cpu.a)?;
    cpu.dec_hl();
    cpu.pc += 1;
    Ok(8)
}

// opcode:     0x3E
// mnemonic:   LD A, n8
// T-cycles:   8
// Flags ZNHC: - - - -
fn ld_a_n8(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    let value = bus.read(cpu.pc + 1)?;
    log::debug!("{}: LD A, {value}", cpu.pc);
    cpu.a = value;
    cpu.pc += 2;
    Ok(8)
}

// opcode:     0xAF
// mnemonic:   XOR A, A
// T-cycles:   4
// Flags ZNHC: 1 0 0 0
fn xor_a_a(cpu: &mut CPU) -> Result<u32> {
    log::debug!("{}: XOR A, A", cpu.pc);

    // A XOR A = 0
    cpu.a = Byte::ZERO;

    // Flags: Z N H C = 1 0 0 0
    cpu.f = Flags::new(true, false, false, false);

    cpu.pc += 1;
    Ok(4)
}

fn cb(cpu: &mut CPU, bus: &mut Bus) -> Result<u32> {
    let opcode = bus.read(cpu.pc + 1)?;
    match opcode.get() {
        0x7C => bit(cpu, 7, cpu.h, "H"),
        _ => Err(Error::CBOpcode(opcode)),
    }
}

// mnemonic:   BIT ?, ?
// T-cycles:   8
// Flags ZNHC: Z 0 1 -
fn bit(cpu: &mut CPU, bit: u8, reg: Byte, name: &str) -> Result<u32> {
    log::debug!("{}: BIT {bit}, {name}", cpu.pc);
    let set = reg.get() & (1 << bit) != 0;
    cpu.f = cpu
        .f
        .with(Flag::Zero, !set)
        .with(Flag::Subtract, false)
        .with(Flag::Half, true);
    cpu.pc += 2;
    Ok(8)
}
