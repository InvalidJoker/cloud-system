use api::{
    context::Context,
    event::{EventHandler, EventManager},
    plugin::Plugin,
    register_plugin,
};
use async_trait::async_trait;

struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        Self
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn test_event(&self, ctx: Context, message: String) {
        println!("Received Test Event: {}", message);
    }
}

#[async_trait]
impl Plugin for ExamplePlugin {
    async fn name(&self) -> String {
        "Example Plugin".to_string()
    }

    async fn startup(&self, manager: &mut EventManager) {
        manager.event_handler(Handler);

        println!("Starting up Example Plugin");
    }
}
register_plugin!(ExamplePlugin);
