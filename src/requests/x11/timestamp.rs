#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timestamp(pub u32);

impl Timestamp {
    pub const CURRENT_TIME: Timestamp = Timestamp(0);
}