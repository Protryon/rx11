use super::*;

impl X11Connection {
    pub async fn list_extensions(&self) -> Result<Vec<String>> {
        let reply = send_request!(self, parse_reserved ListExtensionsReply, ListExtensions {}).into_inner();
        Ok(reply.names.into_iter().map(|x| x.str).collect())
    }

    pub async fn query_extension(&self, name: &str) -> Result<QueryExtensionReply> {
        let reply = send_request!(
            self,
            QueryExtensionReply,
            QueryExtension {
                name: name.to_string(),
            }
        );
        Ok(reply.into_inner())
    }
}
