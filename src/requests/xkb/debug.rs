use super::*;

use crate::coding::xkb::SetDebuggingFlagsRequest;
pub use crate::coding::xkb::SetDebuggingFlagsResponse;

impl X11Connection {
    /// buttons is Some(start, length) or None for all
    pub async fn xkb_set_debugging_flags(
        &self,
        flags: impl Into<Affect<u32>>,
        controls: impl Into<Affect<u32>>,
        message: impl AsRef<str>,
    ) -> Result<SetDebuggingFlagsResponse> {
        let flags = flags.into();
        let controls = controls.into();
        let reply = send_request_xkb!(
            self,
            XKBOpcode::SetDebuggingFlags,
            SetDebuggingFlagsResponse,
            SetDebuggingFlagsRequest {
                affect_flags: flags.affect,
                flags: flags.value,
                affect_ctrls: controls.affect,
                ctrls: controls.value,
                message: message.as_ref().to_string(),
            }
        )
        .into_inner();

        Ok(reply)
    }
}
