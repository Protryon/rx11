use super::*;

pub use crate::coding::{
    HostFamily,
    Host,
};

#[derive(Clone, Debug)]
pub struct HostList {
    pub hosts: Vec<Host>,
    pub acl_enabled: bool,
}

impl X11Connection {
    pub async fn add_acl_host(&self, host: Host) -> Result<()> {
        send_request!(self, InsertDelete::Insert as u8, ChangeHosts {
            host: host,
        });
        Ok(())
    }

    pub async fn remove_acl_host(&self, host: Host) -> Result<()> {
        send_request!(self, InsertDelete::Delete as u8, ChangeHosts {
            host: host,
        });
        Ok(())
    }

    pub async fn list_hosts(&self) -> Result<HostList> {
        let seq = send_request!(self, ListHosts {
        });
        let (reply, acl_enabled) = receive_reply!(self, seq, ListHostsReply, fetched);
        Ok(HostList {
            hosts: reply.hosts,
            acl_enabled: acl_enabled != 0,
        })
    }

    pub async fn set_acl_enabled(&self, enabled: bool) -> Result<()> {
        send_request!(self, enabled as u8, SetAccessControl {
        });
        Ok(())
    }
}