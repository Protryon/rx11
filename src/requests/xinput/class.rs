use bitvec::{prelude::BitVec, order::Lsb0};

use crate::coding::xinput2::{self, DeviceClassData, DeviceClassType};

use super::*;

#[derive(Clone, Debug)]
pub struct DeviceButton {
    pub number: u16,
    pub name: Atom,
    pub is_down: bool,
}

#[derive(Clone, Debug)]
pub struct DeviceValuator {
    pub number: u16,
    pub label: Atom,
    pub min: I32F32,
    pub max: I32F32,
    pub value: I32F32,
    pub resolution: u32,
    pub mode: ValuatorMode,
}

#[derive(Clone, Debug)]
pub struct DeviceScroll {
    pub number: u16,
    pub scroll_type: ScrollType,
    pub flags: ScrollFlags,
    pub increment: I32F32,
}

#[derive(Clone, Debug)]
pub struct DeviceTouch {
    pub mode: TouchMode,
    pub num_touches: u8,
}

#[derive(Debug, Clone)]
pub struct DeviceClass {
    pub buttons: Vec<DeviceButton>,
    pub keys: Vec<u32>,
    pub valuators: Vec<DeviceValuator>,
    pub scrolls: Vec<DeviceScroll>,
    pub touchs: Vec<DeviceTouch>,
}

impl DeviceClass {
    pub(crate) async fn parse(connection: &X11Connection, classes: Vec<xinput2::DeviceClass>) -> Result<Self> {
        let mut buttons = vec![];
        let mut keys = vec![];
        let mut valuators = vec![];
        let mut scrolls = vec![];
        let mut touchs = vec![];

        for class in classes {
            match class.data {
                DeviceClassData::Key { keys: class_keys, .. } => {
                    keys = class_keys
                },
                DeviceClassData::Button { num_buttons, state, label_atoms } => {
                    let state = BitVec::<u32, Lsb0>::from_vec(state);
                    for i in 0..num_buttons {
                        buttons.push(DeviceButton {
                            number: i,
                            name: connection.get_atom_name(label_atoms[i as usize]).await?,
                            //todo: check bit endianness is correct
                            is_down: state[i as usize],
                        });
                    }
                },
                DeviceClassData::Valuator { number, label_atom, min, max, value, resolution, mode } => {
                    valuators.push(DeviceValuator {
                        number,
                        label: connection.get_atom_name(label_atom).await?,
                        min: min.into(),
                        max: max.into(),
                        value: value.into(),
                        resolution,
                        mode,
                    });
                },
                DeviceClassData::Scroll { number, scroll_type, flags, increment } => {
                    scrolls.push(DeviceScroll {
                        number,
                        scroll_type,
                        flags,
                        increment: increment.into(),
                    });
                },
                DeviceClassData::Touch { mode, num_touches } => {
                    touchs.push(DeviceTouch {
                        mode,
                        num_touches,
                    });
                },
                DeviceClassData::Unknown(_) => {},
            }
        }
        Ok(Self {
            buttons,
            keys,
            valuators,
            scrolls,
            touchs,
        })
    }

    pub(crate) fn encode(self, device: Device<'_>) -> Vec<xinput2::DeviceClass> {
        let mut out = vec![];
        if !self.buttons.is_empty() {
            let mut state = BitVec::<u32, Lsb0>::new();
            let mut label_atoms = vec![];
            for button in &self.buttons {
                assert_eq!(state.len(), button.number as usize);
                state.push(button.is_down);
                label_atoms.push(button.name.handle);
            }
            out.push(xinput2::DeviceClass {
                type_: DeviceClassType::Button,
                len: 0,
                source_device: device.id,
                data: DeviceClassData::Button {
                    state: state.into_vec(),
                    label_atoms,
                    num_buttons: 0,
                },
            });
        }
        if !self.keys.is_empty() {
            out.push(xinput2::DeviceClass {
                type_: DeviceClassType::Key,
                len: 0,
                source_device: device.id,
                data: DeviceClassData::Key {
                    keys: self.keys,
                    num_keys: 0,
                },
            });
        }
        for valuator in self.valuators {
            out.push(xinput2::DeviceClass {
                type_: DeviceClassType::Valuator,
                len: 0,
                source_device: device.id,
                data: DeviceClassData::Valuator {
                    number: valuator.number,
                    label_atom: valuator.label.handle,
                    min: valuator.min.into(),
                    max: valuator.max.into(),
                    value: valuator.value.into(),
                    resolution: valuator.resolution,
                    mode: valuator.mode,
                },
            });
        }
        for scroll in self.scrolls {
            out.push(xinput2::DeviceClass {
                type_: DeviceClassType::Scroll,
                len: 0,
                source_device: device.id,
                data: DeviceClassData::Scroll {
                    number: scroll.number,
                    scroll_type: scroll.scroll_type,
                    flags: scroll.flags,
                    increment: scroll.increment.into(),
                },
            });
        }
        for touch in self.touchs {
            out.push(xinput2::DeviceClass {
                type_: DeviceClassType::Touch,
                len: 0,
                source_device: device.id,
                data: DeviceClassData::Touch {
                    mode: touch.mode,
                    num_touches: touch.num_touches,
                },
            });
        }
        out
    }
}