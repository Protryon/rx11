use super::*;

pub use crate::coding::{Host, HostFamily};

#[derive(Clone, Debug)]
pub struct HostList {
    pub hosts: Vec<Host>,
    pub acl_enabled: bool,
}

impl X11Connection {
    pub async fn add_acl_host(&self, host: Host) -> Result<()> {
        send_request!(self, reserved InsertDelete::Insert as u8, ChangeHosts {
            host: host,
        });
        Ok(())
    }

    pub async fn remove_acl_host(&self, host: Host) -> Result<()> {
        send_request!(self, reserved InsertDelete::Delete as u8, ChangeHosts {
            host: host,
        });
        Ok(())
    }

    pub async fn list_acl_hosts(&self) -> Result<HostList> {
        let reply = send_request!(self, ListHostsReply, ListHosts {});
        Ok(HostList {
            acl_enabled: reply.reserved != 0,
            hosts: reply.into_inner().hosts,
        })
    }

    pub async fn set_acl_enabled(&self, enabled: bool) -> Result<()> {
        send_request!(self, reserved enabled as u8, SetAccessControl {});
        Ok(())
    }
}
