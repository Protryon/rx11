use super::*;

impl X11Connection {
    pub async fn list_extensions(&self) -> Result<Vec<String>> {
        let seq = send_request!(self, ListExtensions {});
        let reply = receive_reply!(self, seq, ListExtensionsReply, doubled);
        Ok(reply.names.into_iter().map(|x| x.str).collect())
    }

    pub async fn query_extension(&self, name: &str) -> Result<QueryExtensionReply> {
        let seq = send_request!(self, QueryExtension {
            name: name.to_string(),
        });
        let reply = receive_reply!(self, seq, QueryExtensionReply);
        Ok(reply)
    }
}