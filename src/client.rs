use crate::{
    error::ClientResult,
    http::HttpClient,
    model::{info::Info, namespace::NamespaceHandler},
};
use http::Request;

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

    pub async fn info(&self) -> ClientResult<Info> {
        let req = Request::builder()
            .method("GET")
            .uri(&format!("{}/v1/info", self.base_uri))
            .body(())?;
        let info = self.http_client.send(req).await?;
        Ok(info)
    }

    pub fn namespace(&self, id: &str, token: &str) -> NamespaceHandler {
        NamespaceHandler::new(id, token, self)
    }
}
