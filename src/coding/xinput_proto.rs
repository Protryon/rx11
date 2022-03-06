use self::xinput2::{XIEventCode, XIEventData};

// pub mod xinput1 {
//     protospec::include_spec!("xinput1");
// }
pub mod xinput2 {
    protospec::include_spec!("xinput2");
}

impl XIEventData {
    pub fn code(&self) -> XIEventCode {
        match self {
            XIEventData::DeviceChanged(_) => XIEventCode::DeviceChanged,
            XIEventData::KeyPress(_) => XIEventCode::KeyPress,
            XIEventData::KeyRelease(_) => XIEventCode::KeyRelease,
            XIEventData::ButtonPress(_) => XIEventCode::ButtonPress,
            XIEventData::ButtonRelease(_) => XIEventCode::ButtonRelease,
            XIEventData::Motion(_) => XIEventCode::Motion,
            XIEventData::Enter(_) => XIEventCode::Enter,
            XIEventData::Leave(_) => XIEventCode::Leave,
            XIEventData::FocusIn(_) => XIEventCode::FocusIn,
            XIEventData::FocusOut(_) => XIEventCode::FocusOut,
            XIEventData::Hierarchy(_) => XIEventCode::Hierarchy,
            XIEventData::Property(_) => XIEventCode::Property,
            XIEventData::RawKeyPress(_) => XIEventCode::RawKeyPress,
            XIEventData::RawKeyRelease(_) => XIEventCode::RawKeyRelease,
            XIEventData::RawButtonPress(_) => XIEventCode::RawButtonPress,
            XIEventData::RawButtonRelease(_) => XIEventCode::RawButtonRelease,
            XIEventData::RawMotion(_) => XIEventCode::RawMotion,
            XIEventData::TouchBegin(_) => XIEventCode::TouchBegin,
            XIEventData::TouchUpdate(_) => XIEventCode::TouchUpdate,
            XIEventData::TouchEnd(_) => XIEventCode::TouchEnd,
            XIEventData::TouchOwnership(_) => XIEventCode::TouchOwnership,
            XIEventData::RawTouchBegin(_) => XIEventCode::RawTouchBegin,
            XIEventData::RawTouchUpdate(_) => XIEventCode::RawTouchUpdate,
            XIEventData::RawTouchEnd(_) => XIEventCode::RawTouchEnd,
            XIEventData::BarrierHit(_) => XIEventCode::BarrierHit,
            XIEventData::BarrierLeave(_) => XIEventCode::BarrierLeave,
        }
    }
}
