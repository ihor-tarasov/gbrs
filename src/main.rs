use gbrs::{Bios, Bus, Cpu};

fn init_bus() -> Bus {
    let bios = Bios::load("dmg_boot.bin").unwrap();
    Bus::new(bios)
}

fn main() {
    let mut bus = init_bus();
    let mut cpu = Cpu::new();
    loop {
        cpu.step(&mut bus);
    }
}
