
#[derive(Clone, Copy, Debug)]
pub enum DeviceSpec {
    Id(u8),
    UseCoreKeyboard,
    UseCorePointer,
}

impl Into<u16> for DeviceSpec {
    fn into(self) -> u16 {
        match self {
            DeviceSpec::Id(id) => id as u16,
            DeviceSpec::UseCoreKeyboard => 0x100,
            DeviceSpec::UseCorePointer => 0x200,
        }
    }
}
