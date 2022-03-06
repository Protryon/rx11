use std::collections::BTreeMap;

use crate::coding::xinput2::{self, DeviceClassData};

use super::*;

#[derive(Debug, Clone)]
pub struct DeviceClass {
    pub buttons: BTreeMap<u16, DeviceButton>,
    pub keys: Vec<u32>,
    pub valuators: Vec<DeviceValuator>,
    pub scrolls: Vec<DeviceScroll>,
    pub touches: Vec<DeviceTouch>,
}

impl DeviceClass {
    pub(crate) async fn parse(connection: &X11Connection, classes: Vec<xinput2::DeviceClass>) -> Result<Self> {
        let mut buttons = BTreeMap::new();
        let mut keys = vec![];
        let mut valuators = vec![];
        let mut scrolls = vec![];
        let mut touches = vec![];

        for class in classes {
            match class.data {
                DeviceClassData::Key { keys: class_keys, .. } => {
                    keys = class_keys
                },
                DeviceClassData::Button { num_buttons, state, label_atoms } => {
                    for i in 0..num_buttons {
                        buttons.insert(i, DeviceButton {
                            number: i,
                            name: connection.get_atom_name(label_atoms[i as usize]).await?,
                            //todo: check bit endianness is correct
                            is_down: (state[i as usize / 32] & (0x80000000 >> i as usize)) != 0,
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
                    touches.push(DeviceTouch {
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
            touches,
        })
    }

    pub(crate) fn encode(self) -> Vec<xinput2::DeviceClass> {
        // let mut out = vec![];
        // if !self.buttons.is_empty() {

        // }
        // out
        //TODO:
        unimplemented!()
    }
}