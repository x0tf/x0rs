#[test]
fn test_info() {
    use crate::{model::info::Info, Client};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    smol::block_on(async {
        let info_expected = Info {
            invites: false,
            production: false,
            version: "MOCK".to_string(),
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
fn test_user_agent() {
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
        client.info().await.ok();
        // this just sends a request to the info endpoint,
        // what really matters though is that a request with the correct User-Agent
        // is sent, so the request result is discarded
    })
}
