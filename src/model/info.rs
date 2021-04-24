use crate::{error::ClientResult, Client};
use http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NamespaceIdRules {
    pub min_length: u64,
    pub max_length: u64,
    pub allowed_characters: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub invites: bool,
    pub namespace_id_rules: NamespaceIdRules,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Info {
    pub version: String,
    pub production: bool,
    pub settings: Settings,
}

pub struct InfoHandler<'a> {
    client: &'a Client,
}

impl<'a> InfoHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        InfoHandler { client }
    }

    pub async fn get(&self) -> ClientResult<Info> {
        let client = self.client;
        let req = Request::builder()
            .method("GET")
            .uri(&format!("{}/v2/info", client.base_uri))
            .body(())?;
        let info = client.http_client.send(req).await?;
        Ok(info)
    }
}

#[cfg(test)]
mod test {
    #[async_std::test]
    async fn get_info() {
        use super::*;
        use crate::Client;
        use httpmock::MockServer;

        let info_expected = Info {
            version: "mock-version".to_string(),
            production: false,
            settings: Settings {
                invites: false,
                namespace_id_rules: NamespaceIdRules {
                    min_length: 1,
                    max_length: 32,
                    allowed_characters: "mock-characters".to_string(),
                },
            },
        };

        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path("/v2/info");
                then.status(200).json_body_obj(&info_expected);
            })
            .await;

        let client = Client::new(&server.base_url()).unwrap();
        let info = client.info().get().await.unwrap();

        assert_eq!(info, info_expected);
        mock.assert_async().await;
    }
}
