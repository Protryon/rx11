
#[derive(Clone, Copy, Debug)]
pub struct Keysym(pub u32);

impl Keysym {
    pub const NO_SYMBOL: Keysym = Keysym(0);
    pub const VOID_SYMBOL: Keysym = Keysym(0x00FFFFFF);
}