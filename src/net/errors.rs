use super::*;

#[derive(Debug, Clone)]
pub enum X11ErrorCode {
    X11(ErrorCode),
    XKB(XKBErrorCode),
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

impl std::error::Error for X11ErrorReply {

}

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

impl std::error::Error for X11Error {

}

impl From<anyhow::Error> for X11Error {
    fn from(from: anyhow::Error) -> Self {
        X11Error::Error(from)
    }
}
