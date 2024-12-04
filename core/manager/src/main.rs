use api::{
    context::Context,
    event::{EventManager, FullEvent},
};
use config::ManagerConfig;
use plugins::PluginManager;
use tokio::fs::File;

mod config;
mod plugins;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();

    let mut config_buffer = String::new();

    let mut file = File::open("config.toml").await?;
    tokio::io::AsyncReadExt::read_to_string(&mut file, &mut config_buffer).await?;
    let config: ManagerConfig = toml::from_str(&config_buffer)?;

    println!("{:?}", config);

    manager.load_from_directory("./data")?;

    println!("{}", manager.plugins.len());

    let mut event_manager = EventManager::new();

    let name = manager.plugins[0].name().await;

    for plugin in &manager.plugins {
        plugin.startup(&mut event_manager).await;
    }

    println!("{}", event_manager.handlers.len());

    let ctx = Context::new();
    event_manager
        .dispatch(
            FullEvent::Test {
                message: "Hello".to_string(),
            },
            ctx,
        )
        .await;

    println!("{}", name);

    Ok(())
}
