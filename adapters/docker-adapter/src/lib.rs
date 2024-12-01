use adapter_api::{register_adapter, Adapter};
use async_trait::async_trait;

struct DockerAdapter;

impl DockerAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Adapter for DockerAdapter {
    async fn name(&self) -> String {
        "Docker Adapter".to_string()
    }

    async fn create_server(&self, name: &str) -> String {
        format!("Created server: {}", name)
    }
}

register_adapter!(DockerAdapter);
