use async_trait::async_trait;
use plugin_api::{register_plugin, Plugin};

struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Plugin for ExamplePlugin {
    async fn name(&self) -> String {
        "Example Plugin".to_string()
    }

    async fn execute(&self, input: &str) -> String {
        format!("Processed: {}", input)
    }
}

register_plugin!(ExamplePlugin);
