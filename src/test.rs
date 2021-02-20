#[test]
fn info() {
    use crate::{model::info::Info, Client};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    smol::block_on(async {
        let info_expected = Info {
            invites: false,
            production: false,
            version: "mock-version".to_string(),
        };

        let server = MockServer::start().await;
        let server_uri = server.uri();
        Mock::given(method("GET"))
            .and(path("/v1/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&info_expected))
            .mount(&server)
            .await;

        let client = Client::new(&server_uri).unwrap();
        let info = client.info().await.unwrap();
        assert_eq!(info, info_expected);
    })
}

#[test]
fn user_agent() {
    use crate::Client;
    use wiremock::{matchers::header, Mock, MockServer, ResponseTemplate};

    smol::block_on(async {
        let server = MockServer::start().await;
        let server_uri = server.uri();
        Mock::given(header(
            "User-Agent",
            format!("x0rs / {}", env!("CARGO_PKG_VERSION")).as_str(),
        ))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

        let client = Client::new(&server_uri).unwrap();
        client.info().await.unwrap_err();
        // this just sends a request to the info endpoint,
        // what really matters though is that a request with the correct User-Agent
        // is sent, so the request result is discarded
    })
}

#[test]
fn get_namespace() {
    use crate::{model::namespace::Namespace, Client};
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    smol::block_on(async {
        let namespace_expected = Namespace {
            id: "mock-namespace".to_string(),
            active: true,
        };

        let server = MockServer::start().await;
        let server_uri = server.uri();
        Mock::given(method("GET"))
            .and(path("/v1/namespaces/mock-namespace"))
            .and(header("Authorization", "Bearer mock-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&namespace_expected))
            .mount(&server)
            .await;

        let client = Client::new(&server_uri).unwrap();
        let namespace = client
            .namespace("mock-namespace", "mock-token")
            .get()
            .await
            .unwrap();
        assert_eq!(namespace, namespace_expected);
    })
}

#[test]
fn create_namespace() {
    use crate::Client;
    use serde_json::json;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    smol::block_on(async {
        let server = MockServer::start().await;
        let server_uri = server.uri();

        // the first Mock returns the new namespace with a token,
        // the second Mock checks if the token was saved successfully
        Mock::given(method("POST"))
            .and(path("/v1/namespaces/mock-namespace"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                json!({ "id": "mock-namespace", "token": "mock-token", "active": true }),
            ))
            .mount(&server)
            .await;
        Mock::given(header("Authorization", "Bearer mock-token"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::new(&server_uri).unwrap();
        let mut namespace = client.namespace("mock-namespace", "");
        namespace.create(None).await.unwrap();
        namespace.get().await.unwrap_err();
    })
}

#[test]
fn reset_namespace_token() {
    use crate::Client;
    use serde_json::json;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    smol::block_on(async {
        let server = MockServer::start().await;
        let server_uri = server.uri();

        // the first Mock returns a new token,
        // the second Mock checks if the token was saved successfully
        Mock::given(method("POST"))
            .and(path("/v1/namespaces/mock-namespace/resettoken"))
            .and(header("Authorization", "Bearer mock-token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "token": "mock-token-new" })),
            )
            .mount(&server)
            .await;
        Mock::given(header("Authorization", "Bearer mock-token-new"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::new(&server_uri).unwrap();
        let mut namespace = client.namespace("mock-namespace", "mock-token");
        namespace.reset_token().await.unwrap();
        namespace.get().await.unwrap_err();
    })
}
