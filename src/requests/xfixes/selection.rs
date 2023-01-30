use crate::coding::xfixes::SelectSelectionInputRequest;
pub use crate::coding::xfixes::SelectionEventMask;

use super::*;

impl<'a> Window<'a> {
    pub async fn select_selection_input(self, selection: Atom, event_mask: SelectionEventMask) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::SelectSelectionInput,
            SelectSelectionInputRequest {
                window: self.handle,
                selection_atom: selection.handle,
                event_mask: event_mask,
            }
        );

        Ok(())
    }
}
