use crate::{Bus, Cpu, Error, Result};

pub fn execute(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let opcode = bus.read_byte(cpu.pc)?;
    match opcode.get() {
        0x31 => ld_sp_n16(cpu, bus),
        _ => Err(Error::Opcode(opcode)),
    }
}

// opcode:   0x31
// mnemonic: LD SP, n16
// T-cycles: 12
// Flags:    - - - -
fn ld_sp_n16(cpu: &mut Cpu, bus: &mut Bus) -> Result<u32> {
    let n16 = bus.read_word(cpu.pc + 1)?;
    println!("{}: LD SP, {n16}", cpu.pc);
    cpu.sp = n16;
    cpu.pc += 3;
    Ok(12)
}
