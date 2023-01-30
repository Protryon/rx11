use bitvec::{order::Lsb0, prelude::BitVec};
use derive_builder::Builder;

use super::*;

pub use crate::coding::{GetPointerControlReply, LedMode, OffOnDefault, SetMappingStatus};

#[derive(Default, Builder, Debug, Clone)]
#[builder(default)]
pub struct KeyboardControlParams {
    #[builder(setter(into, strip_option), default)]
    pub key_click_percent: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub bell_percent: Option<i8>,
    #[builder(setter(into, strip_option), default)]
    pub bell_pitch: Option<i16>,
    #[builder(setter(into, strip_option), default)]
    pub bell_duration: Option<i16>,
    #[builder(setter(into, strip_option), default)]
    pub led: Option<u32>,
    #[builder(setter(into, strip_option), default)]
    pub led_mode: Option<LedMode>,
    #[builder(setter(into, strip_option), default)]
    pub key: Option<u8>,
    #[builder(setter(into, strip_option), default)]
    pub auto_repeat_mode: Option<OffOnDefault>,
}

#[derive(Debug, Clone)]
pub struct KeyboardControl {
    pub global_auto_repeat: bool,
    pub led_mask: u32,
    pub key_click_percent: u8,
    pub bell_percent: u8,
    pub bell_pitch: u16,
    pub bell_duration: u16,
    pub auto_repeats: BitVec<u8, Lsb0>,
}

impl X11Connection {
    pub async fn legacy_query_keymap(&self) -> Result<BitVec<u8, Lsb0>> {
        let reply = send_request!(self, QueryKeymapReply, QueryKeymap {});

        Ok(BitVec::from_vec(reply.into_inner().keys))
    }

    pub async fn legacy_change_keyboard_mapping(&self, first_keycode: u8, keysyms: Vec<Vec<Keysym>>) -> Result<()> {
        let keysyms_per_keycode = keysyms.get(0).map(|x| x.len()).unwrap_or(0);
        if keysyms_per_keycode > u8::MAX as usize {
            bail!("cannot have >255 keysyms per keycode");
        }
        if !keysyms.is_empty() && !keysyms.iter().all(|x| x.len() == keysyms_per_keycode) {
            bail!("non-square keysyms shape");
        }
        let keycode_count = keysyms.len() / (keysyms_per_keycode as usize);
        if keycode_count > u8::MAX as usize {
            bail!("cannot have >255 keycodes");
        }
        send_request!(self, reserved keycode_count as u8, ChangeKeyboardMapping {
            first_keycode: first_keycode,
            keysyms_per_keycode: keysyms_per_keycode as u8,
            keysyms: keysyms.into_iter().flatten().map(|x| x.0).collect(),
        });
        Ok(())
    }

    pub async fn legacy_get_keyboard_mapping(&self, first_keycode: u8, count: u8) -> Result<Vec<Vec<Keysym>>> {
        let reply = send_request!(
            self,
            GetKeyboardMappingReply,
            GetKeyboardMapping {
                first_keycode: first_keycode,
                count: count,
            }
        );
        let keysyms_per_keycode = reply.reserved;
        let reply = reply.into_inner();
        Ok(reply
            .keysyms
            .chunks_exact(keysyms_per_keycode as usize)
            .map(|x| x.iter().copied().map(Keysym).collect())
            .collect())
    }

    pub async fn legacy_change_keyboard_control(&self, params: KeyboardControlParams) -> Result<()> {
        let mut bitmask = ChangeKeyboardControlBitmask::default();
        if params.key_click_percent.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::KEY_CLICK_PERCENT;
        }
        if params.bell_percent.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::BELL_PERCENT;
        }
        if params.bell_pitch.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::BELL_PITCH;
        }
        if params.bell_duration.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::BELL_DURATION;
        }
        if params.led.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::LED;
        }
        if params.led_mode.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::LED_MODE;
        }
        if params.key.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::KEY;
        }
        if params.auto_repeat_mode.is_some() {
            bitmask |= ChangeKeyboardControlBitmask::AUTO_REPEAT_MODE;
        }

        send_request!(
            self,
            ChangeKeyboardControl {
                bitmask: bitmask,
                key_click_percent: params.key_click_percent,
                bell_percent: params.bell_percent,
                bell_pitch: params.bell_pitch,
                bell_duration: params.bell_duration,
                led: params.led,
                led_mode: params.led_mode,
                key: params.key,
                auto_repeat_mode: params.auto_repeat_mode,
            }
        );
        Ok(())
    }

    pub async fn legacy_get_keyboard_control(&self) -> Result<KeyboardControl> {
        let reply = send_request!(self, GetKeyboardControlReply, GetKeyboardControl {});
        let global_auto_repeat = reply.reserved;
        let reply = reply.into_inner();
        Ok(KeyboardControl {
            global_auto_repeat: global_auto_repeat != 0,
            led_mask: reply.led_mask,
            key_click_percent: reply.key_click_percent,
            bell_percent: reply.bell_percent,
            bell_pitch: reply.bell_pitch,
            bell_duration: reply.bell_duration,
            auto_repeats: BitVec::from_vec(reply.auto_repeats),
        })
    }

    pub async fn legacy_bell(&self, percent: i8) -> Result<()> {
        send_request!(self, reserved percent as u8, Bell {
        });
        Ok(())
    }

    pub async fn legacy_change_pointer_control(
        &self,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: bool,
        do_threshold: bool,
    ) -> Result<()> {
        send_request!(
            self,
            ChangePointerControl {
                acceleration_numerator: acceleration_numerator,
                acceleration_denominator: acceleration_denominator,
                threshold: threshold,
                do_acceleration: do_acceleration,
                do_threshold: do_threshold,
            }
        );
        Ok(())
    }

    pub async fn legacy_get_pointer_control(&self) -> Result<GetPointerControlReply> {
        let reply = send_request!(self, GetPointerControlReply, GetPointerControl {});
        Ok(reply.into_inner())
    }

    pub async fn legacy_set_pointer_mapping(&self, map: Vec<u8>) -> Result<SetMappingStatus> {
        if map.len() > u8::MAX as usize {
            bail!("map max len is 255");
        }
        let reply = send_request!(self, reserved map.len() as u8, SetPointerMappingReply, SetPointerMapping {
            map: map,
        });
        Ok(SetMappingStatus::decode_sync(&mut &[reply.reserved][..])?)
    }

    pub async fn legacy_get_pointer_mapping(&self) -> Result<Vec<u8>> {
        let reply = send_request!(self, parse_reserved GetPointerMappingReply, GetPointerMapping {
        });
        Ok(reply.into_inner().map)
    }

    pub async fn legacy_set_modifier_mapping(&self, keycodes_per_modifier: u8, keycodes: Vec<u8>) -> Result<bool> {
        let reply = send_request!(self, reserved keycodes_per_modifier, SetModifierMappingReply, SetModifierMapping {
            keycodes: keycodes,
        });
        Ok(reply.reserved != 0)
    }

    pub async fn legacy_get_modifier_mapping(&self) -> Result<Vec<Vec<u8>>> {
        let reply = send_request!(self, parse_reserved GetModifierMappingReply, GetModifierMapping {
        });
        let keycodes_per_modifier = reply.reserved;
        Ok(reply
            .into_inner()
            .keycodes
            .chunks_exact(keycodes_per_modifier as usize)
            .map(|x| x.to_vec())
            .collect())
    }
}
