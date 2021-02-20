use crate::{error::ClientResult, http::HttpClient, model::info::Info};
use http::Request;

pub struct Client<'a> {
    base_uri: &'a str,
    http_client: HttpClient,
}

impl<'a> Client<'a> {
    pub fn new(base_uri: &'a str) -> ClientResult<Self> {
        let client = Client {
            base_uri: base_uri,
            http_client: HttpClient::new()?,
        };
        Ok(client)
    }

    pub async fn info(&self) -> ClientResult<Info> {
        let req = Request::builder()
            .method("GET")
            .uri(&format!("{}/v1/info", self.base_uri))
            .body(())?; // TODO: remove this unwrap?;
        let info = self.http_client.send(req).await?;
        Ok(info)
    }
}
