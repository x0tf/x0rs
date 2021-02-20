use crate::{error::ClientResult, Client};
use http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Namespace {
    pub id: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
struct CreateNamespaceRequest<'a> {
    invite: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    token: String,
}

pub struct NamespaceHandler<'a> {
    id: String,
    token: String,
    client: &'a Client,
}

impl<'a> NamespaceHandler<'a> {
    pub(crate) fn new(id: &str, token: &str, client: &'a Client) -> Self {
        let id = id.to_string();
        let token = token.to_string();
        NamespaceHandler { id, token, client }
    }

    pub async fn get(&self) -> ClientResult<Namespace> {
        let client = self.client;
        let req = Request::builder()
            .method("GET")
            .uri(&format!("{}/v1/namespaces/{}", client.base_uri, self.id))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(())?;
        let namespace = client.http_client.send(req).await?;
        Ok(namespace)
    }

    pub async fn create<'b>(&mut self, invite: Option<&'b str>) -> ClientResult<()> {
        let client = self.client;
        let body = CreateNamespaceRequest { invite };
        let req = Request::builder()
            .method("POST")
            .uri(&format!("{}/v1/namespaces/{}", client.base_uri, self.id))
            .body(body)?;
        let token_response: TokenResponse = client.http_client.send(req).await?;
        self.token = token_response.token;
        Ok(())
    }

    pub async fn reset_token(&mut self) -> ClientResult<()> {
        let client = self.client;
        let req = Request::builder()
            .method("POST")
            .uri(&format!(
                "{}/v1/namespaces/{}/resettoken",
                client.base_uri, self.id
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(())?;
        let token_response: TokenResponse = client.http_client.send(req).await?;
        self.token = token_response.token;
        Ok(())
    }
}
