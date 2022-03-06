use dashmap::mapref::multiple::RefMulti;

use super::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ExtInfo {
    pub major_opcode: u8,
    pub event_start: u8,
    pub error_start: u8,
    pub event_count: u8,
}

impl X11Connection {
    pub(crate) fn get_ext_info(&self, ext_name: &str) -> Option<ExtInfo> {
        self.0.registered_extensions.get(ext_name).map(|x| *x.value())
    }

    pub(crate) fn get_ext_info_by_opcode(&self, opcode: u8) -> Option<RefMulti<'_, String, ExtInfo>> {
        self.0.registered_extensions.iter()
            .find(|entry| entry.value().major_opcode == opcode)
    }

    pub(crate) fn get_ext_info_by_event_code(&self, code: u8) -> Option<RefMulti<'_, String, ExtInfo>> {
        self.0.registered_extensions.iter()
            .find(|entry| {
                let value = entry.value();
                value.event_start <= code && value.event_start + value.event_count > code
            })
    }
}
