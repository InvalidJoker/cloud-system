use config::ManagerConfig;
use libloading::{Library, Symbol};
use plugin_api::Plugin;
use std::sync::Arc;
use tokio::fs::File;

mod config;

pub struct PluginManager {
    plugins: Vec<Arc<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(path)? };

        let create_plugin: Symbol<extern "Rust" fn() -> Box<dyn Plugin>> =
            unsafe { lib.get(b"create_plugin")? };

        let plugin = create_plugin();

        self.plugins.push(Arc::from(plugin));

        Ok(())
    }

    pub async fn execute_all(&self, input: &str) -> Vec<String> {
        let mut results = Vec::new();
        for plugin in &self.plugins {
            results.push(plugin.execute(input).await);
        }
        results
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();

    let mut config_buffer = String::new();

    let mut file = File::open("config.toml").await?;
    tokio::io::AsyncReadExt::read_to_string(&mut file, &mut config_buffer).await?;
    let config: ManagerConfig = toml::from_str(&config_buffer)?;

    println!("{:?}", config);

    manager.load_plugin("test_plugin.dll")?;

    println!("{}", manager.plugins.len());

    let name = manager.plugins[0].name().await;
    println!("{}", name);

    let results = manager.execute_all("Hello, Rust!").await;
    for result in results {
        println!("{}", result);
    }

    Ok(())
}
