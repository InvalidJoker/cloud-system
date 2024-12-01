use libloading::{Library, Symbol};
use plugin_api::Plugin;
use std::sync::Arc;

pub struct PluginManager {
    plugins: Vec<Arc<dyn Plugin>>,
    _libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            _libraries: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(path)?;
            let create_plugin: Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> =
                lib.get(b"create_plugin")?;

            // Create the plugin instance
            let plugin = Arc::from_raw(create_plugin());

            // Add plugin and keep the library alive
            self.plugins.push(plugin);
            self._libraries.push(lib);

            Ok(())
        }
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
