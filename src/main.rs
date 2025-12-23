use gbrs::{Bios, Bus, Cpu};

fn init_bus() -> Bus {
    let bios = Bios::load("dmg_boot.bin").unwrap();
    Bus::new(bios)
}

fn main() {
    let mut bus = init_bus();
    let mut cpu = Cpu::default();
    loop {
        match gbrs::execute(&mut cpu, &mut bus) {
            Ok(_cycles) => {}
            Err(error) => {
                eprintln!("Error: {error}");
                break;
            }
        }
    }
}
