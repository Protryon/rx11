use super::*;

impl X11Connection {
    pub async fn set_selection_owner(&self, window: Window<'_>, selection: Atom, time: Timestamp) -> Result<()> {
        send_request!(
            self,
            SetSelectionOwner {
                window: window.handle,
                selection: selection.handle,
                time: time.0,
            }
        );
        Ok(())
    }

    pub async fn get_selection_owner(&self, selection: Atom) -> Result<Window<'_>> {
        let reply = send_request!(
            self,
            GetSelectionOwnerReply,
            GetSelectionOwner {
                selection: selection.handle,
            }
        );

        Ok(Window {
            handle: reply.window,
            connection: self,
        })
    }

    pub async fn convert_selection(&self, window: Window<'_>, selection: Atom, target: Atom, property: Option<Atom>, time: Timestamp) -> Result<()> {
        send_request!(
            self,
            ConvertSelection {
                window: window.handle,
                selection: selection.handle,
                target: target.handle,
                property: property.map(|x| x.handle).unwrap_or(0),
                time: time.0,
            }
        );
        Ok(())
    }
}
