use super::*;

pub use crate::coding::GetScreenSaverReply;

#[derive(Clone, Debug)]
pub struct Screensaver {
    pub timeout: u16,
    pub interal: u16,
    pub prefer_blanking: bool,
    pub allow_exposures: bool,
}

impl X11Connection {
    pub async fn set_screensaver(
        &self,
        timeout: i16,
        interval: i16,
        prefer_blanking: OffOnDefault,
        allow_exposures: OffOnDefault,
    ) -> Result<()> {
        send_request!(
            self,
            SetScreenSaver {
                timeout: timeout,
                interval: interval,
                prefer_blanking: prefer_blanking,
                allow_exposures: allow_exposures,
            }
        );
        Ok(())
    }

    pub async fn get_screensaver(&self) -> Result<GetScreenSaverReply> {
        let seq = send_request!(self, GetScreenSaver {});
        let reply = receive_reply!(self, seq, GetScreenSaverReply);
        Ok(reply)
    }

    pub async fn force_screensaver(&self, on: bool) -> Result<()> {
        send_request!(self, on as u8, ForceScreenSaver {});
        Ok(())
    }
}
