use crate::{error::ClientResult, Client};
use http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Namespace {
    pub id: String,
    pub active: bool,
    pub created: u64, // TODO: parse as Chrono DateTime?
}

#[derive(Debug, Serialize)]
struct CreateNamespaceRequest<'a> {
    id: &'a str,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
// The API returns the whole namespace here,
// this is not used though, as the create function
// is supposed to be used in a chain
struct TokenResponse {
    token: String,
}

pub struct NamespaceHandler<'a> {
    id: String,
    token: String,
    client: &'a Client,
}

impl<'a> NamespaceHandler<'a> {
    pub(crate) fn new(id: &str, client: &'a Client) -> Self {
        let id = id.to_string();
        let token = String::new();
        NamespaceHandler { id, token, client }
    }

    pub async fn get(&self) -> ClientResult<Namespace> {
        let client = self.client;
        let req = Request::builder()
            .method("GET")
            .uri(&format!("{}/v2/namespaces/{}", client.base_uri, self.id))
            .header("Authorization", format!("Bearer {}", self.token))
            .body(())?;
        let namespace = client.http_client.send(req).await?;
        Ok(namespace)
    }

    pub async fn create(mut self) -> ClientResult<NamespaceHandler<'a>> {
        let client = self.client;
        let body = CreateNamespaceRequest { id: &self.id };
        let req = Request::builder()
            .method("POST")
            .uri(&format!("{}/v2/namespaces", client.base_uri))
            .body(body)?;
        let token_response: TokenResponse = client.http_client.send(req).await?;
        self.token = token_response.token;
        Ok(self)
    }

    pub fn token(mut self, token: &str) -> NamespaceHandler<'a> {
        self.token = token.to_string();
        self
    }
}

#[cfg(test)]
mod test {
    #[async_std::test]
    async fn get_namespace() {
        use super::*;
        use crate::Client;
        use httpmock::MockServer;

        let namespace_expected = Namespace {
            id: "mock-namespace".to_string(),
            active: true,
            created: 0,
        };

        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method("GET")
                    .path("/v2/namespaces/mock-namespace")
                    .header("Authorization", "Bearer mock-token");
                then.status(200).json_body_obj(&namespace_expected);
            })
            .await;

        let client = Client::new(&server.base_url()).unwrap();
        let namespace = client
            .namespace("mock-namespace")
            .token("mock-token")
            .get()
            .await
            .unwrap();

        assert_eq!(namespace, namespace_expected);
        mock.assert_async().await;
    }

    #[async_std::test]
    async fn create_namespace() {
        use super::*;
        use crate::Client;
        use httpmock::MockServer;

        let token_response = TokenResponse {
            token: "mock-token".to_string(),
        };

        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method("POST").path("/v2/namespaces");
                then.status(201).json_body_obj(&token_response);
            })
            .await;

        let client = Client::new(&server.base_url()).unwrap();
        let namespace_handler = client.namespace("mock-namespace").create().await.unwrap();

        assert_eq!(namespace_handler.token, "mock-token".to_string());
        mock.assert_async().await;
    }
}
