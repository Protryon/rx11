pub use crate::coding::{xfixes::XFErrorCode, xinput2::XIErrorCode, xkb::XKBErrorCode, xrecord::XRecordErrorCode, ErrorCode};
use crate::requests::{XFIXES_EXT_NAME, XINPUT_EXT_NAME, XKB_EXT_NAME, XRECORD_EXT_NAME};

use super::*;

#[derive(Debug, Clone)]
pub enum X11ErrorCode {
    X11(ErrorCode),
    XKB(XKBErrorCode),
    XI(XIErrorCode),
    XF(XFErrorCode),
    XRecord(XRecordErrorCode),
    Unknown(u8),
}

impl X11ErrorCode {
    pub(crate) fn from_raw(connection: &X11Connection, code: u8) -> Self {
        if let Ok(code) = ErrorCode::from_repr(code) {
            return X11ErrorCode::X11(code);
        }
        if let Some(xkb) = connection.get_ext_info(XKB_EXT_NAME) {
            if code == xkb.error_start {
                return X11ErrorCode::XKB(XKBErrorCode::Keyboard);
            }
        }
        if let Some(xinput) = connection.get_ext_info(XINPUT_EXT_NAME) {
            if code == xinput.error_start {
                return X11ErrorCode::XI(XIErrorCode::BadDevice);
            }
        }
        if let Some(xfixes) = connection.get_ext_info(XFIXES_EXT_NAME) {
            if code == xfixes.error_start {
                return X11ErrorCode::XF(XFErrorCode::BadRegion);
            }
        }
        if let Some(xrecord) = connection.get_ext_info(XRECORD_EXT_NAME) {
            if code == xrecord.error_start {
                return X11ErrorCode::XRecord(XRecordErrorCode::RecordContext);
            }
        }
        X11ErrorCode::Unknown(code)
    }
}

#[derive(Debug, Clone)]
pub struct X11ErrorReply {
    pub bad_value: u32,
    pub code: X11ErrorCode,
}

impl fmt::Display for X11ErrorReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x11 error: {:?} <{}>", self.code, self.bad_value)
    }
}

impl std::error::Error for X11ErrorReply {}

pub enum X11Error {
    Error(anyhow::Error),
    X11Error(X11ErrorReply),
}

impl fmt::Debug for X11Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X11Error::Error(e) => write!(f, "{:?}", e),
            X11Error::X11Error(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for X11Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X11Error::Error(e) => write!(f, "{}", e),
            X11Error::X11Error(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for X11Error {}

impl From<anyhow::Error> for X11Error {
    fn from(from: anyhow::Error) -> Self {
        X11Error::Error(from)
    }
}
