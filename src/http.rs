use crate::error::{ClientError, ClientResult};
use http::{Request, StatusCode};
use isahc::{AsyncReadResponseExt, HttpClient as IsahcClient};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) struct HttpClient {
    internal: IsahcClient,
}

impl HttpClient {
    pub(crate) fn new() -> ClientResult<Self> {
        let internal = IsahcClient::builder()
            .default_header(
                "User-Agent",
                format!("x0rs / {}", env!("CARGO_PKG_VERSION")),
            )
            .build()?;
        let http_client = HttpClient { internal };
        Ok(http_client)
    }

    pub(crate) async fn send<T>(&self, request: Request<impl Serialize>) -> ClientResult<T>
    where
        T: DeserializeOwned + Unpin,
    {
        let (parts, body) = request.into_parts();
        let body_serialized = serde_json::to_vec(&body)?;
        let request_serialized = Request::from_parts(parts, body_serialized);

        let mut response = self.internal.send_async(request_serialized).await?;
        match response.status() {
            StatusCode::OK => {
                let body_deserialized = response.json().await?;
                Ok(body_deserialized)
            }
            status => Err(ClientError::UnexpectedStatus(status)),
        }
    }
}
