use surge_sdk::{Config, SurgeSdk};

pub struct TestServer {
    pub server: mockito::ServerGuard,
    pub client: SurgeSdk,
}

impl TestServer {
    pub async fn new() -> Self {
        let server = mockito::Server::new_async().await;
        let config = Config::new(server.url(), "0.1.0").unwrap();
        let client = SurgeSdk::new(config).unwrap();
        Self { server, client }
    }

    pub async fn mock_endpoint(&mut self, method: &str, path: &str) -> mockito::Mock {
        self.server.mock(method, path).create_async().await
    }
}
