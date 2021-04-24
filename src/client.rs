use crate::{
    error::ClientResult,
    http::HttpClient,
    model::{info::InfoHandler, namespace::NamespaceHandler},
};

pub struct Client {
    pub base_uri: String,
    pub(crate) http_client: HttpClient,
}

impl Client {
    pub fn new(base_uri: &str) -> ClientResult<Self> {
        let client = Client {
            base_uri: base_uri.to_string(),
            http_client: HttpClient::new()?,
        };
        Ok(client)
    }

    pub fn info(&self) -> InfoHandler {
        InfoHandler::new(self)
    }

    pub fn namespace(&self, id: &str, token: &str) -> NamespaceHandler {
        NamespaceHandler::new(id, token, self)
    }
}
