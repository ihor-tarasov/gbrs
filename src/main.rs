use gbrs::{BIOS, Bus, CPU, PPU};

fn init_bus() -> Bus {
    let bios = BIOS::load("dmg_boot.bin").unwrap();
    Bus {
        bios,
        ppu: PPU::new(),
    }
}

fn main() {
    simple_logger::init().unwrap();
    let mut bus = init_bus();
    let mut cpu = CPU::default();
    loop {
        match gbrs::execute(&mut cpu, &mut bus) {
            Ok(_cycles) => {}
            Err(error) => {
                log::error!("Error: {error}");
                break;
            }
        }
    }
}
