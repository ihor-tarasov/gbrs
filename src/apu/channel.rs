#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Channel {
    Square1,
    Square2,
    Wave,
    Noise,
}
